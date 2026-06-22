use std::marker::PhantomData;

use ark_ec::{AffineRepr, CurveGroup, pairing::Pairing};
use ark_ff::{One, UniformRand};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use chacha20poly1305::{
    ChaCha20Poly1305,
    aead::{Aead, KeyInit, Payload, generic_array::GenericArray},
};
use ferveo_common::serialization::{self, serialize_g1};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, digest::Digest};
use zeroize::{ZeroizeOnDrop, Zeroizing};

#[cfg(feature = "parity-codec")]
use ferveo_common::serialization::parity_codec_helpers::{
    decode_g1, decode_g2, encode_g1, encode_g2,
};
#[cfg(feature = "parity-codec")]
use parity_scale_codec::{Decode, Encode, Error as CodecError, Input, Output};

use crate::{
    Codec, DkgPublicKey, Error, PrivateKeyShare, Result, SharedSecret,
    htp_bls12381_g2,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Hash)]
#[serde(bound(serialize = "", deserialize = ""))]
pub struct Ciphertext<E: Pairing, T = Raw> {
    // U
    #[serde(with = "serialization::ark_serde_configured")]
    pub commitment: E::G1Affine,
    // W
    #[serde(with = "serialization::ark_serde_configured")]
    pub auth_tag: E::G2Affine,
    /// The ciphertext itself.
    /// Created using [chacha20poly1305::ChaCha20Poly1305::encrypt].
    #[cfg_attr(feature = "serde-hex", serde(with = "const_hex"))]
    pub ciphertext: Vec<u8>,
    /// Inner type the ciphertext bind to.
    #[serde(skip)]
    pub _type: PhantomData<T>,
}

#[cfg(feature = "parity-codec")]
impl<E: Pairing, T> Encode for Ciphertext<E, T> {
    fn encode_to<O: Output + ?Sized>(&self, dest: &mut O) {
        encode_g1::<E, _>(&self.commitment, dest);
        encode_g2::<E, _>(&self.auth_tag, dest);
        self.ciphertext.encode_to(dest);
    }
}

#[cfg(feature = "parity-codec")]
impl<E: Pairing, T> Decode for Ciphertext<E, T> {
    fn decode<I: Input>(
        input: &mut I,
    ) -> core::result::Result<Self, CodecError> {
        let commitment = decode_g1::<E, _>(input)?;
        let auth_tag = decode_g2::<E, _>(input)?;
        let ciphertext = <Vec<u8> as Decode>::decode(input)?;
        Ok(Self { commitment, auth_tag, ciphertext, _type: PhantomData })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Raw;

pub type RawCiphertext<E> = Ciphertext<E, Raw>;

impl<E: Pairing, T> Ciphertext<E, T> {
    pub fn check(&self, aad: &[u8]) -> Result<bool> {
        self.header().check(aad)
    }

    pub fn ciphertext_hash(&self) -> [u8; 32] {
        sha256(&self.ciphertext)
    }

    pub fn header(&self) -> CiphertextHeader<E> {
        CiphertextHeader {
            commitment: self.commitment,
            auth_tag: self.auth_tag,
            ciphertext_hash: self.ciphertext_hash(),
        }
    }

    pub fn payload(&self) -> Vec<u8> {
        self.ciphertext.clone()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(bound(serialize = "", deserialize = ""))]
pub struct CiphertextHeader<E: Pairing> {
    #[serde(with = "serialization::ark_serde_configured")]
    pub commitment: E::G1Affine,
    #[serde(with = "serialization::ark_serde_configured")]
    pub auth_tag: E::G2Affine,
    pub ciphertext_hash: [u8; 32],
}

impl<E: Pairing> CiphertextHeader<E> {
    pub fn check(&self, aad: &[u8]) -> Result<bool> {
        // Implements a variant of the check in section 4.4.2 of the Ferveo paper:
        // 'TPKE.CheckCiphertextValidity(U,W,aad)'
        // See: https://eprint.iacr.org/2022/898.pdf
        // See: https://nikkolasg.github.io/ferveo/tpke.html#to-validate-ciphertext-for-ind-cca2-security

        // H_G2(U, sym_ctxt_digest, aad)
        let hash_g2 = E::G2Prepared::from(construct_tag_hash::<E>(
            self.commitment,
            &self.ciphertext_hash,
            aad,
        )?);

        let g = E::G1Affine::generator();
        let g_inv = E::G1Prepared::from(-g.into_group());
        let is_ciphertext_valid = E::multi_pairing(
            // e(U, H_G2(U, sym_ctxt_digest, aad)) == e(G, W) ==>
            // e(U, H_G2(U, sym_ctxt_digest, aad)) * e(G_inv, W) == 1
            [self.commitment.into(), g_inv],
            [hash_g2, self.auth_tag.into()],
        )
        .0 == E::TargetField::one();

        if is_ciphertext_valid {
            Ok(true)
        } else {
            Err(Error::CiphertextVerificationFailed)
        }
    }
}

/// Encodes a typed plaintext that implements [Codec] trait, then encrypt it.
/// Use `decrypt_value` to recover the typed value after share combination.
pub fn encrypt<E: Pairing, T: Codec>(
    message: &T,
    aad: &[u8],
    pubkey: &DkgPublicKey<E>,
    rng: &mut impl rand::Rng,
) -> Result<Ciphertext<E, T>> {
    let encoded = message.encode()?;
    encrypt_raw_bytes(&encoded, aad, pubkey, rng)
}

/// Encrypt byte-like plaintext with associated data under the DKG public key.
/// The plaintext type is tracked in [Ciphertext] but only bytes are encrypted.
pub fn encrypt_raw<E: Pairing>(
    message: impl AsRef<[u8]>,
    aad: &[u8],
    pubkey: &DkgPublicKey<E>,
    rng: &mut impl rand::Rng,
) -> Result<RawCiphertext<E>> {
    encrypt_raw_bytes(message.as_ref(), aad, pubkey, rng)
}

/// Inner helper function that encrypts given slice of bytes.
///
/// Internally it encrypts the data using [chacha20poly1305::ChaCha20Poly1305]
/// AEAD algorithm.
fn encrypt_raw_bytes<E: Pairing, T>(
    message: &[u8],
    aad: &[u8],
    pubkey: &DkgPublicKey<E>,
    rng: &mut impl rand::Rng,
) -> Result<Ciphertext<E, T>> {
    // r - random element to encrypt message with
    let r = E::ScalarField::rand(rng);
    // G1 group generator
    let g1 = E::G1Affine::generator();
    // h
    let h_gen = E::G2Affine::generator();

    let ry_prep = E::G1Prepared::from((pubkey.0 * r).into_affine());
    // s
    let product = E::pairing(ry_prep, h_gen).0;
    // U - public R value
    let commitment = (g1 * r).into_affine();

    let nonce = Nonce::from_commitment::<E>(commitment)?;
    let shared_secret = SharedSecret::<E>(product);

    let payload = Payload { msg: message, aad };
    let ciphertext = shared_secret_to_chacha(&shared_secret)?
        .encrypt(&nonce.0, payload) // TODO: Consider encrypt_in_place (#196)
        .map_err(Error::SymmetricEncryptionError)?
        .to_vec();
    let ciphertext_hash = sha256(&ciphertext);

    // w
    let auth_tag =
        (construct_tag_hash::<E>(commitment, &ciphertext_hash, aad)? * r)
            .into_affine();

    // TODO: Consider adding aad to the Ciphertext struct
    Ok(Ciphertext::<E, T> {
        commitment,
        ciphertext,
        auth_tag,
        _type: PhantomData,
    })
}

/// Typed wrapper function over [decrypt_with_shared_secret].
/// Decrypts given [Ciphertext] and then try to decode it.
pub fn decrypt<E, T>(
    ciphertext: &Ciphertext<E, T>,
    aad: &[u8],
    shared_secret: &SharedSecret<E>,
) -> Result<T>
where
    E: Pairing,
    T: Codec,
{
    let plaintext = decrypt_with_shared_secret(ciphertext, aad, shared_secret)?;
    T::decode(&plaintext)
}

/// Decrypt with a combined threshold shared secret and return plaintext bytes.
/// This is the low-level byte API for interoperable callers.
pub fn decrypt_raw<E: Pairing>(
    ciphertext: &RawCiphertext<E>,
    aad: &[u8],
    shared_secret: &SharedSecret<E>,
) -> Result<Vec<u8>> {
    decrypt_with_shared_secret(ciphertext, aad, shared_secret)
}

/// Typed wrapper function over [decrypt_symmetric].
/// Decrypts given [Ciphertext] and then tries to decode it.
pub fn decrypt_symmetric<E: Pairing, T: Codec>(
    ciphertext: &Ciphertext<E, T>,
    aad: &[u8],
    private_key: &PrivateKeyShare<E>,
) -> Result<T> {
    let shared_secret = E::pairing(
        E::G1Prepared::from(ciphertext.commitment),
        E::G2Prepared::from(private_key.0),
    )
    .0;
    let shared_secret = SharedSecret(shared_secret);
    let plaintext =
        decrypt_with_shared_secret(ciphertext, aad, &shared_secret)?;
    T::decode(&plaintext)
}

/// Decrypt directly with the private key and return plaintext bytes.
/// This is mainly useful for non-threshold local checks and tests.
pub fn decrypt_symmetric_raw<E: Pairing>(
    ciphertext: &RawCiphertext<E>,
    aad: &[u8],
    private_key: &PrivateKeyShare<E>,
) -> Result<Vec<u8>> {
    let shared_secret = E::pairing(
        E::G1Prepared::from(ciphertext.commitment),
        E::G2Prepared::from(private_key.0),
    )
    .0;
    let shared_secret = SharedSecret(shared_secret);
    decrypt_with_shared_secret(ciphertext, aad, &shared_secret)
}

fn decrypt_with_shared_secret<E: Pairing, T>(
    ciphertext: &Ciphertext<E, T>,
    aad: &[u8],
    shared_secret: &SharedSecret<E>,
) -> Result<Vec<u8>> {
    ciphertext.check(aad)?;
    let nonce = Nonce::from_commitment::<E>(ciphertext.commitment)?;
    let ctxt = ciphertext.ciphertext.to_vec();
    let payload = Payload { msg: ctxt.as_ref(), aad };
    let plaintext = shared_secret_to_chacha(shared_secret)?
        .decrypt(&nonce.0, payload)
        .map_err(|_| Error::CiphertextVerificationFailed)?
        .to_vec();

    Ok(plaintext)
}

fn sha256(input: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    result.into()
}

pub fn shared_secret_to_chacha<E: Pairing>(
    shared_secret: &SharedSecret<E>,
) -> Result<ChaCha20Poly1305> {
    let mut prf_key = Zeroizing::new(Vec::new());
    shared_secret.0.serialize_compressed(&mut *prf_key)?;
    Ok(ChaCha20Poly1305::new(GenericArray::from_slice(&sha256(
        prf_key.as_slice(),
    ))))
}

/// Wrapper around the Nonce implementation from the `chacha20poly1305` crate.
/// This wrapper implements `ZeroizeOnDrop` to ensure that the key is zeroed when the
/// `Nonce` struct is dropped.
#[derive(ZeroizeOnDrop)]
pub struct Nonce(pub(crate) chacha20poly1305::Nonce);

impl Nonce {
    pub fn from_commitment<E: Pairing>(
        commitment: E::G1Affine,
    ) -> Result<Self> {
        let commitment_bytes = serialize_g1::<E>(&commitment);
        let commitment_hash = sha256(&commitment_bytes);
        Ok(Self(*chacha20poly1305::Nonce::from_slice(&commitment_hash[..12])))
    }
}

fn hash_to_g2<T: CanonicalDeserialize>(message: &[u8]) -> Result<T> {
    let point = htp_bls12381_g2(message);
    let mut point_ser: Vec<u8> = Vec::new();
    point.serialize_compressed(&mut point_ser)?;
    T::deserialize_compressed(&point_ser[..]).map_err(Error::ArkSerializeError)
}

fn construct_tag_hash<E: Pairing>(
    commitment: E::G1Affine,
    ciphertext_hash: &[u8],
    aad: &[u8],
) -> Result<E::G2Affine> {
    let mut hash_input = Vec::<u8>::new();
    commitment.serialize_compressed(&mut hash_input)?;
    hash_input.extend_from_slice(ciphertext_hash);
    hash_input.extend_from_slice(aad);
    hash_to_g2(&hash_input)
}

#[cfg(test)]
mod tests {
    use std::marker::PhantomData;

    use ark_std::test_rng;

    use crate::*;
    type E = ark_bls12_381::Bls12_381;

    #[test]
    fn symmetric_encryption() {
        let rng = &mut test_rng();
        let shares_num = 16;
        let threshold = shares_num * 2 / 3;
        let msg = "my-msg".as_bytes().to_vec();
        let aad: &[u8] = "my-aad".as_bytes();

        let DealerOutput { public_key: pubkey, private_key: privkey, .. } =
            deal::<E>(shares_num, threshold, rng);

        let ciphertext = encrypt_raw::<E>(&msg, aad, &pubkey, rng).unwrap();

        let plaintext =
            decrypt_symmetric_raw(&ciphertext, aad, &privkey).unwrap();

        assert_eq!(msg, plaintext);

        let bad: &[u8] = "bad-aad".as_bytes();

        assert!(decrypt_symmetric_raw(&ciphertext, bad, &privkey).is_err());
    }

    #[test]
    fn ciphertext_validity_check() {
        let rng = &mut test_rng();
        let shares_num = 16;
        let threshold = shares_num * 2 / 3;
        let msg = "my-msg".as_bytes().to_vec();
        let aad: &[u8] = "my-aad".as_bytes();
        let DealerOutput { public_key: pubkey, .. } =
            deal::<E>(shares_num, threshold, rng);
        let mut ciphertext = encrypt_raw::<E>(&msg, aad, &pubkey, rng).unwrap();

        // So far, the ciphertext is valid
        assert!(ciphertext.check(aad).is_ok());

        // Malformed the ciphertext
        ciphertext.ciphertext[0] += 1;
        assert!(ciphertext.check(aad).is_err());

        // Malformed the AAD
        let aad = "bad aad".as_bytes();
        assert!(ciphertext.check(aad).is_err());
    }

    #[test]
    fn ciphertext_serde_correct() {
        let ciphertext = Ciphertext::<E, u64> {
            commitment: <E as Pairing>::G1Affine::default(),
            auth_tag: <E as Pairing>::G2Affine::default(),
            ciphertext: vec![1u8, 2u8, 3u8],
            _type: PhantomData,
        };

        let serialized = serde_json::to_string_pretty(&ciphertext).unwrap();
        let deserialized: Ciphertext<E, u64> =
            serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized, ciphertext);
    }
}
