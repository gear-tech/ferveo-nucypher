#![allow(non_snake_case)]

use ark_ec::hashing::{
    HashToCurve, curve_maps::wb::WBMap,
    map_to_curve_hasher::MapToCurveBasedHasher,
};
use ark_ff::field_hashers::DefaultFieldHasher;
use sha2::Sha256;

/// Domain separation tag for RFC9380 BLS12-381 G2 hash-to-curve.
pub const HTP_BLS12381_G2_DST: &[u8] =
    b"QUUX-V01-CS02-with-BLS12381G2_XMD:SHA-256_SSWU_RO_";

/// Hash to BLS12-381 G2 per RFC9380 with [HTP_BLS12381_G2_DST].
/// Uses ark-ec native SSWU map + Wahby–Boneh isogeny.
pub fn htp_bls12381_g2(msg: &[u8]) -> ark_bls12_381::G2Affine {
    let hasher = MapToCurveBasedHasher::<
        ark_bls12_381::G2Projective,
        DefaultFieldHasher<Sha256, 128>,
        WBMap<ark_bls12_381::g2::Config>,
    >::new(HTP_BLS12381_G2_DST)
    .expect("hash-to-curve hasher init");
    hasher.hash(msg).expect("hash-to-curve")
}

#[cfg(test)]
mod tests {
    use ark_bls12_381::{Fq, Fq2, G2Affine};
    use ark_ff::PrimeField;

    use super::*;

    /// Parse big-endian hex (with optional `0x` prefix) into `Fq`.
    fn fq_from_be_hex(s: &str) -> Fq {
        let s = s.strip_prefix("0x").unwrap_or(s);
        let bytes = hex::decode(s).expect("hex");
        Fq::from_be_bytes_mod_order(&bytes)
    }

    /// RFC9380 vectors give x = (c0, c1), y = (c0, c1) over Fq2.
    fn g2_from_rfc(x_c0: &str, x_c1: &str, y_c0: &str, y_c1: &str) -> G2Affine {
        let x = Fq2::new(fq_from_be_hex(x_c0), fq_from_be_hex(x_c1));
        let y = Fq2::new(fq_from_be_hex(y_c0), fq_from_be_hex(y_c1));
        G2Affine::new(x, y)
    }

    /// RFC9380 §J.10.1 with DST `QUUX-V01-CS02-with-BLS12381G2_XMD:SHA-256_SSWU_RO_`,
    /// msg = "" — expected P from arkworks ships test vectors.
    #[test]
    fn hash_nothing_g2() {
        let expected = g2_from_rfc(
            "0x0141ebfbdca40eb85b87142e130ab689c673cf60f1a3e98d69335266f30d9b8d4ac44c1038e9dcdd5393faf5c41fb78a",
            "0x05cb8437535e20ecffaef7752baddf98034139c38452458baeefab379ba13dff5bf5dd71b72418717047f5b0f37da03d",
            "0x0503921d7f6a12805e72940b963c0cf3471c7b2a524950ca195d11062ee75ec076daf2d4bc358c4b190c0c98064fdd92",
            "0x12424ac32561493f3fe3c260708a12b7c620e7be00099a974e259ddc7d1f6395c3c811cdd19f1e8dbf3e9ecfdcbab8d6",
        );
        assert_eq!(htp_bls12381_g2(b""), expected);
    }

    /// RFC9380 §J.10.1 with same DST, msg = "abc".
    #[test]
    fn hash_abc_g2() {
        let expected = g2_from_rfc(
            "0x02c2d18e033b960562aae3cab37a27ce00d80ccd5ba4b7fe0e7a210245129dbec7780ccc7954725f4168aff2787776e6",
            "0x139cddbccdc5e91b9623efd38c49f81a6f83f175e80b06fc374de9eb4b41dfe4ca3a230ed250fbe3a2acf73a41177fd8",
            "0x1787327b68159716a37440985269cf584bcb1e621d3a7202be6ea05c4cfe244aeb197642555a0645fb87bf7466b2ba48",
            "0x00aa65dae3c8d732d10ecd2c50f8a1baf3001578f71c694e03866e9f3d49ac1e1ce70dd94a733534f106d4cec0eddd16",
        );
        assert_eq!(htp_bls12381_g2(b"abc"), expected);
    }
}
