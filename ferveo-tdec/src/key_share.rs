use std::{collections::HashMap, fmt, ops::Mul, str::FromStr};

use ark_ec::{CurveGroup, pairing::Pairing};
use ark_ff::Field;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use ferveo_common::{Keypair, serialization};
use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::{
    CiphertextHeader, DecryptionSharePrecomputed, DecryptionShareSimple,
    DomainPoint, Error, Result, prepare_combine_simple,
};

/// Public key produced by the dealer setup.
///
/// Its command-line string representation is `0x`-prefixed hex of the
/// canonical compressed G1 point.
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(bound(serialize = "", deserialize = ""))]
pub struct DkgPublicKey<E: Pairing>(
    #[serde(with = "serialization::ark_serde_configured")] pub E::G1Affine,
);

impl<E: Pairing> fmt::Display for DkgPublicKey<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut bytes = Vec::with_capacity(self.0.compressed_size());
        self.0.serialize_compressed(&mut bytes).map_err(|_| fmt::Error)?;
        write!(f, "0x{}", hex::encode(bytes))
    }
}

impl<E: Pairing> FromStr for DkgPublicKey<E> {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self> {
        let bytes = hex::decode(value.strip_prefix("0x").unwrap_or(value))?;
        let mut input = bytes.as_slice();
        let public_key = E::G1Affine::deserialize_compressed(&mut input)?;
        if !input.is_empty() {
            return Err(Error::TrailingBytes(input.len()));
        }
        Ok(Self(public_key))
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(bound(serialize = "", deserialize = ""))]
pub struct ShareCommitment<E: Pairing>(
    #[serde(with = "serialization::ark_serde_configured")] pub E::G1Affine, // A_{i, \omega_i}
);

// TODO: Improve by adding share commitment here
// TODO: Is this a test utility perhaps?
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(bound(serialize = "", deserialize = ""))]
pub struct BlindedKeyShare<E: Pairing> {
    #[serde(with = "serialization::ark_serde_configured")]
    pub validator_public_key: E::G2Affine, // [b] H
    #[serde(with = "serialization::ark_serde_configured")]
    pub blinded_key_share: E::G2Affine, // [b] Z_{i, \omega_i}
}

impl<E: Pairing> BlindedKeyShare<E> {
    pub fn unblind(
        &self,
        validator_keypair: &Keypair<E>,
    ) -> Result<PrivateKeyShare<E>> {
        let unblinding_factor = validator_keypair
            .decryption_key
            .inverse()
            .expect("Validator decryption key must have an inverse");
        Ok(PrivateKeyShare::<E>(
            self.blinded_key_share.mul(unblinding_factor).into_affine(),
        ))
    }

    pub fn create_decryption_share_simple(
        &self,
        ciphertext_header: &CiphertextHeader<E>,
        aad: &[u8],
        validator_keypair: &Keypair<E>,
    ) -> Result<DecryptionShareSimple<E>> {
        DecryptionShareSimple::create(
            &validator_keypair.decryption_key,
            &self.unblind(validator_keypair)?,
            ciphertext_header,
            aad,
        )
    }

    /// In precomputed variant, we offload some of the decryption related computation to the server-side:
    /// We use the `prepare_combine_simple` function to precompute the lagrange coefficients
    pub fn create_decryption_share_precomputed(
        &self,
        ciphertext_header: &CiphertextHeader<E>,
        aad: &[u8],
        validator_keypair: &Keypair<E>,
        share_index: u32,
        domain_points_map: &HashMap<u32, DomainPoint<E>>,
    ) -> Result<DecryptionSharePrecomputed<E>> {
        // We need to turn the domain points into a vector, and sort it by share index
        let mut domain_points = domain_points_map
            .iter()
            .map(|(share_index, domain_point)| (*share_index, *domain_point))
            .collect::<Vec<_>>();
        domain_points.sort_by_key(|(share_index, _)| *share_index);

        // Now, we have to pass the domain points to the `prepare_combine_simple` function
        // and use the resulting lagrange coefficients to create the decryption share

        let only_domain_points = domain_points
            .iter()
            .map(|(_, domain_point)| *domain_point)
            .collect::<Vec<_>>();
        let lagrange_coeffs = prepare_combine_simple::<E>(&only_domain_points);

        // Before we pick the lagrange coefficient for the current share index, we need
        // to map the share index to the index in the domain points vector
        // Given that we sorted the domain points by share index, the first element in the vector
        // will correspond to the smallest share index, second to the second smallest, and so on

        let sorted_share_indices = domain_points
            .iter()
            .enumerate()
            .map(|(adjusted_share_index, (share_index, _))| {
                (*share_index, adjusted_share_index)
            })
            .collect::<HashMap<u32, usize>>();
        let adjusted_share_index =
            *sorted_share_indices.get(&share_index).unwrap();

        // Finally, pick the lagrange coefficient for the current share index
        let lagrange_coeff = &lagrange_coeffs[adjusted_share_index];
        let private_key_share = self.unblind(validator_keypair);
        DecryptionSharePrecomputed::create(
            share_index as usize,
            &validator_keypair.decryption_key,
            &private_key_share.unwrap(),
            ciphertext_header,
            aad,
            lagrange_coeff,
        )
    }
}

#[derive(
    Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Zeroize, ZeroizeOnDrop,
)]
#[serde(bound(serialize = "", deserialize = ""))]
pub struct PrivateKeyShare<E: Pairing>(
    #[serde(with = "serialization::ark_serde_configured")] pub E::G2Affine,
);
