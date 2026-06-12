#![warn(rust_2018_idioms)]
use ark_ec::pairing::Pairing;

mod ciphertext;
mod codec;
mod combine;
mod context;
mod dealer;
mod decryption;
mod hash_to_curve;
mod key_share;

pub use ciphertext::{
    Ciphertext, CiphertextHeader, Raw, RawCiphertext, decrypt, decrypt_raw,
    decrypt_symmetric, decrypt_symmetric_raw, encrypt, encrypt_raw,
};
pub use codec::Codec;
pub use combine::{
    SharedSecret, lagrange_coefficients_at, prepare_combine_simple,
    share_combine_precomputed, share_combine_simple,
};
pub use context::{
    PrivateDecryptionContextSimple, PublicDecryptionContextSimple,
};
pub use dealer::{DealerOutput, create_shared_secret_simple, deal};
pub use decryption::{
    DecryptionSharePrecomputed, DecryptionShareSimple, ValidatorShareChecksum,
    verify_decryption_shares_simple,
};
pub use hash_to_curve::{HTP_BLS12381_G2_DST, htp_bls12381_g2};
pub use key_share::{
    BlindedKeyShare, DkgPublicKey, PrivateKeyShare, ShareCommitment,
};

/// Provides BLS12-381 type aliases.
#[cfg(feature = "bls12_381")]
pub mod bls12_381;

/// Re-exports [rand::Rng] crate.
pub mod rand_utils {
    pub use ark_std::test_rng;
    pub use rand::Rng;
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Ciphertext verification failed
    /// Refers to the check 4.4.2 in the paper: https://eprint.iacr.org/2022/898.pdf
    #[error("Ciphertext verification failed")]
    CiphertextVerificationFailed,

    /// Decryption share verification failed
    /// Refers to the check 4.4.4 in the paper: https://eprint.iacr.org/2022/898.pdf
    #[error("Decryption share verification failed")]
    DecryptionShareVerificationFailed,

    /// Symmetric encryption failed"
    #[error("Symmetric encryption failed")]
    SymmetricEncryptionError(chacha20poly1305::aead::Error),

    #[error(transparent)]
    BincodeError(#[from] bincode::Error),

    #[error(transparent)]
    ArkSerializeError(#[from] ark_serialize::SerializationError),

    #[cfg(feature = "parity-codec")]
    #[error(transparent)]
    ParityCodecError(#[from] parity_scale_codec::Error),
}

pub type DomainPoint<E> = <E as Pairing>::ScalarField;
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(all(test, feature = "parity-codec"))]
mod tests {
    use std::ops::Mul;

    use ark_ec::{CurveGroup, pairing::Pairing};
    use ark_std::{UniformRand, test_rng};
    use ferveo_common::{FromBytes, ToBytes};
    use rand::seq::IteratorRandom;

    use crate::*;

    type E = ark_bls12_381::Bls12_381;
    type TargetField = <E as Pairing>::TargetField;
    type ScalarField = <E as Pairing>::ScalarField;

    #[test]
    fn ciphertext_serialization() {
        let rng = &mut test_rng();
        let shares_num = 16;
        let threshold = shares_num * 2 / 3;
        let msg = "my-msg".as_bytes().to_vec();
        let aad: &[u8] = "my-aad".as_bytes();

        let DealerOutput { public_key: pubkey, .. } =
            deal::<E>(shares_num, threshold, rng);

        let ciphertext = encrypt_raw::<E>(&msg, aad, &pubkey, rng).unwrap();

        let serialized = ciphertext.to_bytes().unwrap();
        let deserialized: Ciphertext<E> =
            Ciphertext::from_bytes(&serialized).unwrap();

        assert_eq!(serialized, deserialized.to_bytes().unwrap())
    }

    fn test_ciphertext_validation_fails<E: Pairing>(
        aad: &[u8],
        ciphertext: &RawCiphertext<E>,
        shared_secret: &SharedSecret<E>,
    ) {
        // Malformed the ciphertext
        let mut ciphertext = ciphertext.clone();
        ciphertext.ciphertext[0] += 1;
        assert!(decrypt_raw(&ciphertext, aad, shared_secret).is_err());

        // Malformed the AAD
        let aad = "bad aad".as_bytes();
        assert!(decrypt_raw(&ciphertext, aad, shared_secret).is_err());
    }

    #[test]
    fn tdec_simple_variant_share_validation() {
        let rng = &mut test_rng();
        let shares_num = 16;
        let threshold = shares_num * 2 / 3;
        let msg = "my-msg".as_bytes().to_vec();
        let aad: &[u8] = "my-aad".as_bytes();

        let DealerOutput {
            public_key: pubkey, private_contexts: contexts, ..
        } = deal::<E>(shares_num, threshold, rng);
        let ciphertext = encrypt_raw::<E>(&msg, aad, &pubkey, rng).unwrap();

        let bad_aad = "bad aad".as_bytes();
        assert!(
            contexts[0]
                .create_share(&ciphertext.header().unwrap(), bad_aad)
                .is_err()
        );
    }

    #[test]
    fn tdec_simple_variant_e2e() {
        let mut rng = &mut test_rng();
        let shares_num = 16;
        let threshold = shares_num * 2 / 3;
        let msg = String::from("my-msg");
        let aad: &[u8] = "my-aad".as_bytes();

        let DealerOutput {
            public_key: pubkey, private_contexts: contexts, ..
        } = deal::<E>(shares_num, threshold, &mut rng);

        let ciphertext = encrypt_raw::<E>(&msg, aad, &pubkey, rng).unwrap();

        // We need at least threshold shares to decrypt
        let decryption_shares: Vec<_> = contexts
            .iter()
            .map(|c| {
                c.create_share(&ciphertext.header().unwrap(), aad).unwrap()
            })
            .take(threshold)
            .collect();
        let selected_contexts =
            contexts[0].public_decryption_contexts[..threshold].to_vec();
        let shared_secret =
            create_shared_secret_simple(&selected_contexts, &decryption_shares);

        let plaintext = decrypt_raw(&ciphertext, aad, &shared_secret).unwrap();
        assert_eq!(plaintext, msg.as_bytes());

        test_ciphertext_validation_fails(aad, &ciphertext, &shared_secret);

        // If we use less than threshold shares, we should fail
        let not_enough_dec_shares = decryption_shares[..threshold - 1].to_vec();
        let not_enough_contexts = selected_contexts[..threshold - 1].to_vec();
        let bash_shared_secret = create_shared_secret_simple(
            &not_enough_contexts,
            &not_enough_dec_shares,
        );
        let result = decrypt_raw(&ciphertext, aad, &bash_shared_secret);
        assert!(result.is_err());
    }

    #[test]
    fn tdec_simple_variant_codec_e2e() {
        let mut rng = &mut test_rng();
        let shares_num = 16;
        let threshold = shares_num * 2 / 3;
        let msg = "my-msg".to_string();
        let aad = "my-aad".as_bytes();

        let DealerOutput {
            public_key: pubkey, private_contexts: contexts, ..
        } = deal::<E>(shares_num, threshold, &mut rng);

        let ciphertext = encrypt::<E, _>(&msg, aad, &pubkey, rng).unwrap();

        let decryption_shares: Vec<_> = contexts
            .iter()
            .map(|c| {
                c.create_share(&ciphertext.header().unwrap(), aad).unwrap()
            })
            .take(threshold)
            .collect();
        let selected_contexts =
            contexts[0].public_decryption_contexts[..threshold].to_vec();
        let shared_secret =
            create_shared_secret_simple(&selected_contexts, &decryption_shares);

        let plaintext = decrypt(&ciphertext, aad, &shared_secret).unwrap();
        assert_eq!(plaintext, msg);
    }

    #[test]
    fn tdec_precomputed_variant_e2e() {
        let mut rng = &mut test_rng();
        let shares_num = 16;
        let threshold = shares_num * 2 / 3;
        let msg = "my-msg".as_bytes().to_vec();
        let aad: &[u8] = "my-aad".as_bytes();

        let DealerOutput {
            public_key: pubkey, private_contexts: contexts, ..
        } = deal::<E>(shares_num, threshold, &mut rng);
        let ciphertext = encrypt_raw::<E>(&msg, aad, &pubkey, rng).unwrap();

        let selected_participants =
            (0..threshold).choose_multiple(rng, threshold);
        let selected_contexts = contexts
            .iter()
            .filter(|c| selected_participants.contains(&c.index))
            .cloned()
            .collect::<Vec<_>>();

        let decryption_shares = selected_contexts
            .iter()
            .map(|context| {
                context
                    .create_share_precomputed(
                        &ciphertext.header().unwrap(),
                        aad,
                        &selected_participants,
                    )
                    .unwrap()
            })
            .collect::<Vec<DecryptionSharePrecomputed<_>>>();

        let shared_secret = share_combine_precomputed::<E>(&decryption_shares);
        let plaintext = decrypt_raw(&ciphertext, aad, &shared_secret).unwrap();
        assert_eq!(plaintext, msg);

        test_ciphertext_validation_fails(aad, &ciphertext, &shared_secret);

        // If we use less than threshold shares, we should fail
        let not_enough_dec_shares = decryption_shares[..threshold - 1].to_vec();
        let bash_shared_secret =
            share_combine_precomputed(&not_enough_dec_shares);
        let result = decrypt_raw(&ciphertext, aad, &bash_shared_secret);
        assert!(result.is_err());
    }

    #[test]
    fn tdec_simple_variant_share_verification() {
        let mut rng = &mut test_rng();
        let shares_num = 16;
        let threshold = shares_num * 2 / 3;
        let msg = "my-msg".as_bytes().to_vec();
        let aad: &[u8] = "my-aad".as_bytes();

        let DealerOutput {
            public_key: pubkey, private_contexts: contexts, ..
        } = deal::<E>(shares_num, threshold, &mut rng);

        let ciphertext = encrypt_raw::<E>(&msg, aad, &pubkey, rng).unwrap();

        let decryption_shares: Vec<_> = contexts
            .iter()
            .map(|c| {
                c.create_share(&ciphertext.header().unwrap(), aad).unwrap()
            })
            .collect();

        // In simple tDec variant, we verify decryption shares only after decryption fails.
        // We could do that before, but we prefer to optimize for the happy path.

        // Let's assume that combination failed here. We'll try to verify decryption shares
        // against validator checksums.

        let pub_contexts = &contexts[0].public_decryption_contexts;
        assert!(verify_decryption_shares_simple(
            pub_contexts,
            &ciphertext,
            &decryption_shares,
        ));

        // Now, let's test that verification fails if we one of the decryption shares is invalid.

        let mut has_bad_checksum = decryption_shares[0].clone();
        has_bad_checksum.validator_checksum.checksum = has_bad_checksum
            .validator_checksum
            .checksum
            .mul(ScalarField::rand(rng))
            .into_affine();

        assert!(!has_bad_checksum.verify(
            &pub_contexts[0].blinded_key_share.blinded_key_share,
            &pub_contexts[0].validator_public_key.encryption_key,
            &ciphertext,
        ));

        let mut has_bad_share = decryption_shares[0].clone();
        has_bad_share.decryption_share =
            has_bad_share.decryption_share.mul(TargetField::rand(rng));

        assert!(!has_bad_share.verify(
            &pub_contexts[0].blinded_key_share.blinded_key_share,
            &pub_contexts[0].validator_public_key.encryption_key,
            &ciphertext,
        ));
    }

    #[test]
    fn ctx_serde() {
        let mut rng = test_rng();
        let deal = deal::<E>(3, 2, &mut rng);
        let ctx = deal.private_contexts.first().unwrap();
        let serialized = serde_json::to_string_pretty(ctx).unwrap();
        println!("{serialized}");
    }
}
