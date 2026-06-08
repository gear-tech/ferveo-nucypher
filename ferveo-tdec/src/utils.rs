pub mod ark_serde_hex {
    use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: CanonicalSerialize,
        S: serde::Serializer,
    {
        let mut bytes = Vec::with_capacity(value.compressed_size());
        value
            .serialize_compressed(&mut bytes)
            .map_err(serde::ser::Error::custom)?;

        const_hex::serialize(bytes, serializer)
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        T: CanonicalDeserialize,
        D: serde::Deserializer<'de>,
    {
        let bytes: Vec<u8> = const_hex::deserialize(deserializer)?;
        T::deserialize_compressed(&mut bytes.as_slice())
            .map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Mul;

    use ark_ec::{AffineRepr, CurveGroup, pairing::Pairing};
    use ark_ff::UniformRand;
    use serde::{Deserialize, Serialize};

    use super::ark_serde_hex;

    type E = ark_bls12_381::Bls12_381;
    type G1Affine = <E as Pairing>::G1Affine;
    type G2Affine = <E as Pairing>::G2Affine;
    type TargetField = <E as Pairing>::TargetField;

    #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
    struct ArkSerdeFixture {
        #[serde(with = "ark_serde_hex")]
        g1: G1Affine,
        #[serde(with = "ark_serde_hex")]
        g2: G2Affine,
        #[serde(with = "ark_serde_hex")]
        gt: TargetField,
    }

    #[test]
    fn ark_bls12_381_points_round_trip_through_hex_json() {
        let rng = &mut ark_std::test_rng();
        let g1 = G1Affine::generator()
            .mul(<E as Pairing>::ScalarField::rand(rng))
            .into_affine();
        let g2 = G2Affine::generator()
            .mul(<E as Pairing>::ScalarField::rand(rng))
            .into_affine();
        let gt = E::pairing(g1, g2).0;
        let fixture = ArkSerdeFixture { g1, g2, gt };

        let serialized = serde_json::to_value(&fixture).unwrap();
        assert!(serialized["g1"].as_str().is_some());
        assert!(serialized["g2"].as_str().is_some());
        assert!(serialized["gt"].as_str().is_some());

        let deserialized: ArkSerdeFixture =
            serde_json::from_value(serialized).unwrap();
        assert_eq!(deserialized, fixture);
    }
}
