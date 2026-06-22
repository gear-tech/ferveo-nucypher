use std::ops::Mul;

use ark_ec::{CurveGroup, PrimeGroup, pairing::Pairing};
use ark_ff::Field;
use ferveo_common::serialization;
use itertools::izip;
use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::{
    Ciphertext, CiphertextHeader, PrivateKeyShare,
    PublicDecryptionContextSimple, Result,
};

#[cfg(feature = "parity-codec")]
use ferveo_common::serialization::parity_codec_helpers::{
    decode_g1, decode_target, encode_g1, encode_target,
};
#[cfg(feature = "parity-codec")]
use parity_scale_codec::{Decode, Encode, Error as CodecError, Output};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(bound(serialize = "", deserialize = ""))]
pub struct ValidatorShareChecksum<E: Pairing> {
    #[serde(with = "serialization::ark_serde_configured")]
    pub checksum: E::G1Affine,
}

impl<E: Pairing> ValidatorShareChecksum<E> {
    pub fn new(
        validator_decryption_key: &E::ScalarField,
        ciphertext_header: &CiphertextHeader<E>,
    ) -> Result<Self> {
        // C_i = dk_i^{-1} * U
        let checksum = ciphertext_header
            .commitment
            .mul(
                validator_decryption_key
                    .inverse()
                    .expect("Inverse of this key doesn't exist"),
            )
            .into_affine();
        Ok(Self { checksum })
    }

    pub fn verify<T>(
        &self,
        decryption_share: &E::TargetField,
        share_aggregate: &E::G2Affine,
        validator_public_key: &E::G2Affine,
        ciphertext: &Ciphertext<E, T>,
    ) -> bool {
        // See https://github.com/nucypher/ferveo/issues/42#issuecomment-1398953777
        // D_i == e(C_i, Y_i)
        if *decryption_share != E::pairing(self.checksum, *share_aggregate).0 {
            return false;
        }

        // TODO: use multipairing here (h_inv) - Issue #192
        // e(C_i, ek_i) == e(U, H)
        if E::pairing(self.checksum, *validator_public_key)
            != E::pairing(ciphertext.commitment, E::G2::generator())
        {
            return false;
        }

        true
    }
}

/// A decryption share for a simple variant of the threshold decryption scheme.
/// In this variant, the decryption share require additional computation on the
/// client side int order to be combined.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(bound(serialize = "", deserialize = ""))]
pub struct DecryptionShareSimple<E: Pairing> {
    #[serde(with = "serialization::ark_serde_configured")]
    pub decryption_share: E::TargetField,
    #[serde(bound(
        serialize = "ValidatorShareChecksum<E>: Serialize",
        deserialize = "ValidatorShareChecksum<E>: DeserializeOwned"
    ))]
    pub validator_checksum: ValidatorShareChecksum<E>,
}

impl<E: Pairing> DecryptionShareSimple<E> {
    /// Create a decryption share from the given parameters.
    /// This function checks that the ciphertext is valid.
    pub fn create(
        validator_decryption_key: &E::ScalarField,
        private_key_share: &PrivateKeyShare<E>,
        ciphertext_header: &CiphertextHeader<E>,
        aad: &[u8],
    ) -> Result<Self> {
        ciphertext_header.check(aad)?;
        Self::create_unchecked(
            validator_decryption_key,
            private_key_share,
            ciphertext_header,
        )
    }

    /// Create a decryption share from the given parameters.
    /// This function does not check that the ciphertext is valid.
    pub fn create_unchecked(
        validator_decryption_key: &E::ScalarField,
        private_key_share: &PrivateKeyShare<E>,
        ciphertext_header: &CiphertextHeader<E>,
    ) -> Result<Self> {
        // D_i = e(U, Z_i)
        let decryption_share =
            E::pairing(ciphertext_header.commitment, private_key_share.0).0;

        let validator_checksum = ValidatorShareChecksum::new(
            validator_decryption_key,
            ciphertext_header,
        )?;

        Ok(Self { decryption_share, validator_checksum })
    }
    /// Verify that the decryption share is valid.
    pub fn verify<T>(
        &self,
        share_aggregate: &E::G2Affine,
        validator_public_key: &E::G2Affine,
        ciphertext: &Ciphertext<E, T>,
    ) -> bool {
        self.validator_checksum.verify(
            &self.decryption_share,
            share_aggregate,
            validator_public_key,
            ciphertext,
        )
    }
}

#[cfg(feature = "parity-codec")]
impl<E: Pairing> Encode for DecryptionShareSimple<E> {
    fn encode_to<T: Output + ?Sized>(&self, dest: &mut T) {
        encode_target::<E, _>(&self.decryption_share, dest);
        encode_g1::<E, _>(&self.validator_checksum.checksum, dest);
    }
}

#[cfg(feature = "parity-codec")]
impl<E: Pairing> Decode for DecryptionShareSimple<E> {
    fn decode<I: parity_scale_codec::Input>(
        input: &mut I,
    ) -> core::result::Result<Self, CodecError> {
        let decryption_share = decode_target::<E, _>(input)?;
        let checksum = decode_g1::<E, _>(input)?;

        Ok(Self {
            decryption_share,
            validator_checksum: ValidatorShareChecksum { checksum },
        })
    }
}

/// A decryption share for a precomputed variant of the threshold decryption scheme.
/// In this variant, the decryption share is precomputed and can be combined
/// without additional computation on the client side.
/// The downside is that the threshold of decryption shares required to decrypt
/// is equal to the number of private key shares in the scheme.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(bound(serialize = "", deserialize = ""))]
pub struct DecryptionSharePrecomputed<E: Pairing> {
    pub decrypter_index: usize,
    #[serde(with = "serialization::ark_serde_configured")]
    pub decryption_share: E::TargetField,
    #[serde(bound(
        serialize = "ValidatorShareChecksum<E>: Serialize",
        deserialize = "ValidatorShareChecksum<E>: DeserializeOwned"
    ))]
    pub validator_checksum: ValidatorShareChecksum<E>,
}

impl<E: Pairing> DecryptionSharePrecomputed<E> {
    /// Create a decryption share from the given parameters.
    /// This function checks that the ciphertext is valid.
    pub fn create(
        validator_index: usize,
        validator_decryption_key: &E::ScalarField,
        private_key_share: &PrivateKeyShare<E>,
        ciphertext_header: &CiphertextHeader<E>,
        aad: &[u8],
        lagrange_coeff: &E::ScalarField,
    ) -> Result<Self> {
        ciphertext_header.check(aad)?;
        Self::create_unchecked(
            validator_index,
            validator_decryption_key,
            private_key_share,
            ciphertext_header,
            lagrange_coeff,
        )
    }

    /// Create a decryption share from the given parameters.
    /// This function does not check that the ciphertext is valid.
    pub fn create_unchecked(
        validator_index: usize,
        validator_decryption_key: &E::ScalarField,
        private_key_share: &PrivateKeyShare<E>,
        ciphertext_header: &CiphertextHeader<E>,
        lagrange_coeff: &E::ScalarField,
    ) -> Result<Self> {
        // U_{λ_i} = [λ_{i}(0)] U
        let u_to_lagrange_coeff =
            ciphertext_header.commitment.mul(lagrange_coeff);
        // C_{λ_i} = e(U_{λ_i}, Z_i)
        let decryption_share =
            E::pairing(u_to_lagrange_coeff, private_key_share.0).0;

        let validator_checksum = ValidatorShareChecksum::new(
            validator_decryption_key,
            ciphertext_header,
        )?;

        Ok(Self {
            decrypter_index: validator_index,
            decryption_share,
            validator_checksum,
        })
    }

    /// Verify that the decryption share is valid.
    pub fn verify<T>(
        &self,
        share_aggregate: &E::G2Affine,
        validator_public_key: &E::G2Affine,
        ciphertext: &Ciphertext<E, T>,
    ) -> bool {
        self.validator_checksum.verify(
            &self.decryption_share,
            share_aggregate,
            validator_public_key,
            ciphertext,
        )
    }
}

pub fn verify_decryption_shares_simple<E: Pairing, T>(
    pub_contexts: &[PublicDecryptionContextSimple<E>],
    ciphertext: &Ciphertext<E, T>,
    decryption_shares: &[DecryptionShareSimple<E>],
) -> bool {
    let blinded_key_shares = &pub_contexts
        .iter()
        .map(|c| &c.blinded_key_share.blinded_key_share)
        .collect::<Vec<_>>();
    for (decryption_share, y_i, pub_context) in
        izip!(decryption_shares, blinded_key_shares, pub_contexts)
    {
        let is_valid = decryption_share.verify(
            y_i,
            &pub_context.validator_public_key.encryption_key,
            ciphertext,
        );
        if !is_valid {
            return false;
        }
    }
    true
}
