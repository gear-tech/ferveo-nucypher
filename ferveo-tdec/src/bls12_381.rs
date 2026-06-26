//! BLS12-381 convenience API.

pub type E = ark_bls12_381::Bls12_381;

pub type G1Prepared = <E as ark_ec::pairing::Pairing>::G1Prepared;

pub type G1Affine = <E as ark_ec::pairing::Pairing>::G1Affine;

pub type G2Affine = <E as ark_ec::pairing::Pairing>::G2Affine;

pub type Fr = ark_bls12_381::Fr;

pub type PrivateKey = ark_bls12_381::G2Affine;

pub type Result<T> = crate::Result<T>;

pub type PrivateDecryptionContextSimple =
    crate::PrivateDecryptionContextSimple<E>;

pub type PublicDecryptionContextSimple =
    crate::PublicDecryptionContextSimple<E>;

pub type DecryptionSharePrecomputed = crate::DecryptionSharePrecomputed<E>;

pub type DecryptionShareSimple = crate::DecryptionShareSimple<E>;

pub type Ciphertext<T = Vec<u8>> = crate::Ciphertext<E, T>;

pub type CiphertextHeader = crate::CiphertextHeader<E>;

pub type TargetField = <E as ark_ec::pairing::Pairing>::TargetField;

pub type DkgPublicKey = crate::DkgPublicKey<E>;

pub type SharedSecret = crate::SharedSecret<E>;

pub type Keypair = ferveo_common::Keypair<E>;

pub type PublicKey = ferveo_common::PublicKey<E>;

pub use crate::{
    Codec, decrypt, decrypt_raw, decrypt_symmetric, encrypt, encrypt_raw,
    prepare_combine_simple, share_combine_precomputed, share_combine_simple,
};

pub use ferveo_common::{from_bytes, to_bytes};
