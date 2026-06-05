use ark_ec::pairing::Pairing;

use crate::{
    BlindedKeyShare, CiphertextHeader, DecryptionSharePrecomputed,
    DecryptionShareSimple, PrivateKeyShare, Result, ShareCommitment,
    prepare_combine_simple,
};

/// Public metadata for one participant in the simple threshold decryption
/// scheme.
///
/// These values can be distributed to clients or aggregators. They identify
/// the participant's point in the secret-sharing domain and provide the public
/// material needed to verify and combine decryption shares.
#[derive(Clone, Debug)]
pub struct PublicDecryptionContextSimple<E: Pairing> {
    /// Participant's evaluation point in the secret-sharing domain.
    ///
    /// This value is used to compute the Lagrange coefficient for this
    /// participant when combining threshold decryption shares.
    pub domain: E::ScalarField,
    /// Public commitment to the participant's private key share.
    ///
    /// In the dealer setup this is `[f(domain)] G`, where `f` is the threshold
    /// polynomial and `G` is the G1 generator.
    pub share_commitment: ShareCommitment<E>,
    /// Participant's private key share blinded by their validator key.
    ///
    /// This is public verification material. The corresponding validator
    /// unblinds it locally to recover the private key share used for
    /// decryption.
    pub blinded_key_share: BlindedKeyShare<E>,
    /// Validator public key used to verify decryption-share checksums.
    pub validator_public_key: ferveo_common::PublicKey<E>,
}

/// Private per-participant context for producing simple threshold decryption
/// shares.
///
/// This context is held by a single participant. It contains that
/// participant's validator decryption key, secret key share, and a copy of
/// the public participant contexts needed to compute Lagrange coefficients
/// for precomputed shares.
#[derive(Clone, Debug)]
pub struct PrivateDecryptionContextSimple<E: Pairing> {
    /// Participant's index in the public context list.
    pub index: usize,
    /// Validator's private scalar used to unblind this participant's key share.
    ///
    /// This is also used when creating the checksum attached to a decryption
    /// share.
    pub validator_decryption_key: E::ScalarField,
    /// Participant's unblinded private key share.
    ///
    /// This value must remain private. It is used to create simple decryption
    /// shares of the form `e(U, Z_i)`, where `U` comes from the ciphertext and
    /// `Z_i` is this share.
    pub private_key_share: PrivateKeyShare<E>,
    /// Public contexts for all participants in the threshold setup.
    ///
    /// This list is used by `create_share_precomputed` to select domain points
    /// and compute the corresponding Lagrange coefficients.
    pub public_decryption_contexts: Vec<PublicDecryptionContextSimple<E>>,
}

impl<E: Pairing> PrivateDecryptionContextSimple<E> {
    pub fn create_share(
        &self,
        ciphertext_header: &CiphertextHeader<E>,
        aad: &[u8],
    ) -> Result<DecryptionShareSimple<E>> {
        DecryptionShareSimple::create(
            &self.validator_decryption_key,
            &self.private_key_share,
            ciphertext_header,
            aad,
        )
    }

    pub fn create_share_precomputed(
        &self,
        ciphertext_header: &CiphertextHeader<E>,
        aad: &[u8],
        selected_participants: &[usize],
    ) -> Result<DecryptionSharePrecomputed<E>> {
        let selected_domain_points = selected_participants
            .iter()
            .map(|i| self.public_decryption_contexts[*i].domain)
            .collect::<Vec<_>>();
        let lagrange_coeffs =
            prepare_combine_simple::<E>(&selected_domain_points);

        DecryptionSharePrecomputed::create(
            self.index,
            &self.validator_decryption_key,
            &self.private_key_share,
            ciphertext_header,
            aad,
            &lagrange_coeffs[self.index],
        )
    }
}
