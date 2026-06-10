//! This adds a few utility functions for serializing and deserializing
//! [arkworks](http://arkworks.rs/) types that implement [CanonicalSerialize] and [CanonicalDeserialize].

use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use serde::{Deserialize, Deserializer, Serialize, Serializer, de, ser};

pub mod ark_serde {
    use core::{
        fmt::{Formatter, Result as FmtResult},
        marker::PhantomData,
    };

    use serde::de::Visitor;

    use super::*;

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: CanonicalSerialize,
        S: Serializer,
    {
        let mut bytes = Vec::with_capacity(value.compressed_size());
        value
            .serialize_compressed(&mut bytes)
            .map_err(ser::Error::custom)?;
        serializer.serialize_bytes(&bytes)
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        T: CanonicalDeserialize,
        D: Deserializer<'de>,
    {
        struct ArkVisitor<T>(PhantomData<T>);

        impl<T: CanonicalDeserialize> Visitor<'_> for ArkVisitor<T> {
            type Value = T;

            fn expecting(&self, formatter: &mut Formatter<'_>) -> FmtResult {
                formatter.write_str(
                    "an Arkworks canonically serialized byte sequence",
                )
            }

            fn visit_bytes<E>(self, bytes: &[u8]) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                T::deserialize_compressed(bytes).map_err(E::custom)
            }

            fn visit_byte_buf<E>(self, bytes: Vec<u8>) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                self.visit_bytes(&bytes)
            }
        }

        deserializer.deserialize_bytes(ArkVisitor(PhantomData))
    }
}

#[cfg(feature = "ark-serde-hex")]
pub mod ark_serde_hex {
    use super::*;

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: CanonicalSerialize,
        S: Serializer,
    {
        let mut bytes = Vec::with_capacity(value.compressed_size());
        value
            .serialize_compressed(&mut bytes)
            .map_err(ser::Error::custom)?;
        const_hex::serialize(bytes, serializer)
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        T: CanonicalDeserialize,
        D: Deserializer<'de>,
    {
        let bytes: Vec<u8> = const_hex::deserialize(deserializer)?;
        T::deserialize_compressed(&mut bytes.as_slice())
            .map_err(de::Error::custom)
    }
}

// TODO: Trait aliases are experimental
// trait ByteSerializable = ToBytes + FromBytes;

pub trait ToBytes {
    fn to_bytes(&self) -> Result<Vec<u8>, bincode::Error>;
}

pub trait FromBytes: Sized {
    fn from_bytes(bytes: &[u8]) -> Result<Self, bincode::Error>;
}

impl<T: Serialize> ToBytes for T {
    fn to_bytes(&self) -> Result<Vec<u8>, bincode::Error> {
        bincode::serialize(self)
    }
}

impl<T: for<'de> Deserialize<'de>> FromBytes for T {
    fn from_bytes(bytes: &[u8]) -> Result<Self, bincode::Error> {
        bincode::deserialize(bytes)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct Test {
        a: u32,
        b: u32,
    }

    #[test]
    fn test_serde() {
        let test = Test { a: 1, b: 2 };
        let bytes = test.to_bytes().unwrap();
        let test2 = Test::from_bytes(&bytes).unwrap();
        assert_eq!(test, test2);
    }
}

#[cfg(all(test, feature = "ark-serde-hex"))]
mod tests_ark_hex {

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
