//! This adds a few utility functions for serializing and deserializing
//! [arkworks](http://arkworks.rs/) types that implement [CanonicalSerialize] and [CanonicalDeserialize].

use alloc::vec::Vec;

use ark_ec::pairing::Pairing;
use ark_serialize::{
    CanonicalDeserialize, CanonicalSerialize, SerializationError,
};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use serde::{Deserializer, Serializer, de, ser};

#[cfg(feature = "parity-codec")]
use parity_scale_codec::{Decode, Encode, Error as CodecError, Input, Output};

fn serialize_point<P: CanonicalSerialize>(point: &P) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(point.compressed_size());
    point
        .serialize_compressed(&mut bytes)
        .expect("serializing to Vec should not fail");
    bytes
}

fn deserialize_point<P: CanonicalDeserialize>(
    bytes: &[u8],
) -> Result<P, SerializationError> {
    P::deserialize_compressed(bytes)
}

pub fn serialize_g1<E: Pairing>(point: &E::G1Affine) -> Vec<u8> {
    serialize_point(point)
}

pub fn deserialize_g1<E: Pairing>(
    bytes: &[u8],
) -> Result<E::G1Affine, SerializationError> {
    deserialize_point(bytes)
}

pub fn serialize_g2<E: Pairing>(point: &E::G2Affine) -> Vec<u8> {
    serialize_point(point)
}

pub fn deserialize_g2<E: Pairing>(
    bytes: &[u8],
) -> Result<E::G2Affine, SerializationError> {
    deserialize_point(bytes)
}

pub fn serialize_target<E: Pairing>(point: &E::TargetField) -> Vec<u8> {
    serialize_point(point)
}

pub fn deserialize_target<E: Pairing>(
    bytes: &[u8],
) -> Result<E::TargetField, SerializationError> {
    deserialize_point(bytes)
}

pub mod ark_serde_default {
    use core::{
        fmt::{Formatter, Result as FmtResult},
        marker::PhantomData,
    };

    use serde::de::{SeqAccess, Visitor};

    use super::*;

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: CanonicalSerialize,
        S: Serializer,
    {
        let mut bytes = Vec::with_capacity(value.compressed_size());
        value.serialize_compressed(&mut bytes).map_err(ser::Error::custom)?;
        serializer.serialize_bytes(&bytes)
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        T: CanonicalDeserialize,
        D: Deserializer<'de>,
    {
        struct ArkVisitor<T>(PhantomData<T>);

        impl<'de, T: CanonicalDeserialize> Visitor<'de> for ArkVisitor<T> {
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

            // Some serializers, including serde_json, encode bytes as a sequence.
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut bytes =
                    Vec::with_capacity(seq.size_hint().unwrap_or_default());
                while let Some(byte) = seq.next_element()? {
                    bytes.push(byte);
                }
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
        value.serialize_compressed(&mut bytes).map_err(ser::Error::custom)?;
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

#[cfg(feature = "parity-codec")]
pub mod parity_codec_helpers {
    use alloc::vec::Vec;

    use super::{
        CanonicalDeserialize, CanonicalSerialize, CodecError, Decode, Encode,
        Input, Output, Pairing, deserialize_point, serialize_point,
    };

    fn encode_point<P: CanonicalSerialize, O: Output + ?Sized>(
        point: &P,
        dest: &mut O,
    ) {
        serialize_point(point).encode_to(dest);
    }

    fn decode_point<P: CanonicalDeserialize, I: Input>(
        input: &mut I,
        error: &'static str,
    ) -> Result<P, CodecError> {
        let bytes = <Vec<u8> as Decode>::decode(input)?;
        deserialize_point(&bytes).map_err(|_| CodecError::from(error))
    }

    pub fn encode_g1<E: Pairing, O: Output + ?Sized>(
        point: &E::G1Affine,
        dest: &mut O,
    ) {
        encode_point(point, dest);
    }

    pub fn decode_g1<E: Pairing, I: Input>(
        input: &mut I,
    ) -> Result<E::G1Affine, CodecError> {
        decode_point(input, "failed to deserialize E::G1Affine")
    }

    pub fn encode_g2<E: Pairing, O: Output + ?Sized>(
        point: &E::G2Affine,
        dest: &mut O,
    ) {
        encode_point(point, dest);
    }

    pub fn decode_g2<E: Pairing, I: Input>(
        input: &mut I,
    ) -> Result<E::G2Affine, CodecError> {
        decode_point(input, "failed to deserialize E::G2Affine")
    }

    pub fn encode_target<E: Pairing, O: Output + ?Sized>(
        point: &E::TargetField,
        dest: &mut O,
    ) {
        encode_point(point, dest);
    }

    pub fn decode_target<E: Pairing, I: Input>(
        input: &mut I,
    ) -> Result<E::TargetField, CodecError> {
        decode_point(input, "failed to deserialize E::TargetField")
    }
}

/// Arkworks serde format selected by the `ark-serde-hex` feature.
#[cfg(feature = "ark-serde-hex")]
pub use ark_serde_hex as ark_serde_configured;

/// Arkworks serde default format.
#[cfg(not(feature = "ark-serde-hex"))]
pub use ark_serde_default as ark_serde_configured;

// TODO: Trait aliases are experimental
// trait ByteSerializable = ToBytes + FromBytes;

#[cfg(feature = "std")]
pub trait ToBytes {
    fn to_bytes(&self) -> Result<Vec<u8>, bincode::Error>;
}

#[cfg(feature = "std")]
pub trait FromBytes: Sized {
    fn from_bytes(bytes: &[u8]) -> Result<Self, bincode::Error>;
}

#[cfg(feature = "std")]
impl<T: Serialize> ToBytes for T {
    fn to_bytes(&self) -> Result<Vec<u8>, bincode::Error> {
        bincode::serialize(self)
    }
}

#[cfg(feature = "std")]
impl<T: for<'de> Deserialize<'de>> FromBytes for T {
    fn from_bytes(bytes: &[u8]) -> Result<Self, bincode::Error> {
        bincode::deserialize(bytes)
    }
}

#[cfg(all(test, feature = "std"))]
mod test {
    use ark_bls12_381::Bls12_381;
    use ark_ec::{AffineRepr, pairing::Pairing};

    use super::*;

    #[cfg(feature = "parity-codec")]
    pub use parity_codec_helpers::*;

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

    #[test]
    fn ark_points_round_trip_through_compressed_bytes() {
        type E = Bls12_381;

        let g1 = <E as Pairing>::G1Affine::generator();
        let g2 = <E as Pairing>::G2Affine::generator();
        let target = E::pairing(g1, g2).0;

        assert_eq!(deserialize_g1::<E>(&serialize_g1::<E>(&g1)).unwrap(), g1);
        assert_eq!(deserialize_g2::<E>(&serialize_g2::<E>(&g2)).unwrap(), g2);
        assert_eq!(
            deserialize_target::<E>(&serialize_target::<E>(&target)).unwrap(),
            target
        );
    }

    #[cfg(feature = "parity-codec")]
    #[test]
    fn ark_points_round_trip_through_scale_codec() {
        type E = Bls12_381;

        let g1 = <E as Pairing>::G1Affine::generator();
        let g2 = <E as Pairing>::G2Affine::generator();
        let target = E::pairing(g1, g2).0;
        let mut encoded = Vec::new();

        encode_g1::<E, _>(&g1, &mut encoded);
        encode_g2::<E, _>(&g2, &mut encoded);
        encode_target::<E, _>(&target, &mut encoded);

        let expected = [
            serialize_g1::<E>(&g1).encode(),
            serialize_g2::<E>(&g2).encode(),
            serialize_target::<E>(&target).encode(),
        ]
        .concat();
        assert_eq!(encoded, expected);

        let mut input = encoded.as_slice();
        assert_eq!(decode_g1::<E, _>(&mut input).unwrap(), g1);
        assert_eq!(decode_g2::<E, _>(&mut input).unwrap(), g2);
        assert_eq!(decode_target::<E, _>(&mut input).unwrap(), target);
        assert!(input.is_empty());
    }
}

#[cfg(all(test, feature = "ark-serde-hex", feature = "std"))]
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
