use ark_ec::{AffineRepr, CurveGroup, pairing::Pairing};
use ark_ff::{Field, UniformRand, Zero};
use ark_poly::{
    DenseUVPolynomial, EvaluationDomain, Polynomial,
    univariate::DensePolynomial,
};
use itertools::izip;
use std::ops::Mul;
use subproductdomain::fast_multiexp;

use crate::{
    BlindedKeyShare, DecryptionShareSimple, DkgPublicKey,
    PrivateDecryptionContextSimple, PrivateKeyShare,
    PublicDecryptionContextSimple, SetupParams, ShareCommitment, SharedSecret,
    prepare_combine_simple, share_combine_simple,
};

#[derive(Clone, Debug)]
pub struct DealerOutput<E: Pairing> {
    /// Group public key.
    pub public_key: DkgPublicKey<E>,
    /// Group private key. This is only useful for non-threshold checks and tests.
    pub private_key: PrivateKeyShare<E>,
    /// Per-participant private decryption contexts.
    pub private_contexts: Vec<PrivateDecryptionContextSimple<E>>,
}

/// Deals a public master key and private decryption contexts for a threshold
/// decryption setup.
pub fn deal<E: Pairing>(
    shares_num: usize,
    threshold: usize,
    rng: &mut impl rand::Rng,
) -> DealerOutput<E> {
    assert!(threshold > 0, "threshold must be greater than zero");
    assert!(
        shares_num >= threshold,
        "number of shares can not be less than threshold"
    );

    let g = E::G1Affine::generator();
    let h = E::G2Affine::generator();

    // The dealer chooses a uniformly random polynomial f of degree t-1.
    let threshold_poly =
        DensePolynomial::<E::ScalarField>::rand(threshold - 1, rng);

    let fft_domain =
        ark_poly::GeneralEvaluationDomain::<E::ScalarField>::new(shares_num)
            .expect("number of shares must fit in an evaluation domain");

    let domain_points = fft_domain.elements().collect::<Vec<_>>();

    // Evaluations of f over the domain: f(omega_j) for omega_j in Omega.
    let evals = threshold_poly.evaluate_over_domain_by_ref(fft_domain);

    // A_j, share commitments of participants: [f(omega_j)] G.
    let share_commitments = fast_multiexp(&evals.evals, g.into_group());

    // Z_j, private key shares of participants (unblinded): [f(omega_j)] H.
    // NOTE: In production, these are never produced this way, as the DKG
    // directly generates blinded shares Y_j. Only then, node j can use their
    // validator key to unblind Y_j and obtain the private key share Z_j.
    let privkey_shares = fast_multiexp(&evals.evals, h.into_group());

    // The shared secret is the free coefficient from threshold poly.
    let a_0 = threshold_poly.coeffs[0];

    // F_0, group's public key.
    let group_pubkey = g * a_0;

    // Group's private key. This is NEVER constructed in production DKG, but
    // callers can use it for non-threshold checks and tests.
    let private_key = PrivateKeyShare::<E>(h.mul(a_0).into());

    // As in SSS, shared secret should be f(0), which is also the free coefficient.
    let secret = threshold_poly.evaluate(&E::ScalarField::zero());
    debug_assert!(secret == a_0);

    let mut private_contexts = vec![];
    let mut public_contexts = vec![];

    for (index, (domain_point, share_commit, private_share)) in izip!(
        domain_points.into_iter(),
        share_commitments.into_iter(),
        privkey_shares.into_iter()
    )
    .enumerate()
    {
        let private_key_share = PrivateKeyShare::<E>(private_share);
        let blinding_factor = E::ScalarField::rand(rng);

        let validator_public_key = h.mul(blinding_factor).into_affine();
        let blinded_key_share = BlindedKeyShare::<E> {
            validator_public_key,
            blinded_key_share: private_key_share
                .0
                .mul(blinding_factor)
                .into_affine(),
        };

        private_contexts.push(PrivateDecryptionContextSimple::<E> {
            index,
            setup_params: SetupParams {
                b: blinding_factor,
                b_inv: blinding_factor.inverse().unwrap(),
                g,
                h_inv: E::G2Prepared::from(-h.into_group()),
                g_inv: E::G1Prepared::from(-g.into_group()),
                h,
            },
            private_key_share,
            public_decryption_contexts: vec![],
        });
        public_contexts.push(PublicDecryptionContextSimple::<E> {
            domain: domain_point,
            share_commitment: ShareCommitment::<E>(share_commit),
            blinded_key_share,
            validator_public_key: ferveo_common::PublicKey {
                encryption_key: blinded_key_share.validator_public_key,
            },
        });
    }
    for private_ctxt in private_contexts.iter_mut() {
        private_ctxt.public_decryption_contexts = public_contexts.clone();
    }

    DealerOutput {
        public_key: DkgPublicKey(group_pubkey.into()),
        private_key,
        private_contexts,
    }
}

pub fn create_shared_secret_simple<E: Pairing>(
    pub_contexts: &[PublicDecryptionContextSimple<E>],
    decryption_shares: &[DecryptionShareSimple<E>],
) -> SharedSecret<E> {
    let domain = pub_contexts.iter().map(|c| c.domain).collect::<Vec<_>>();
    let lagrange_coeffs = prepare_combine_simple::<E>(&domain);
    share_combine_simple::<E>(decryption_shares, &lagrange_coeffs)
}
