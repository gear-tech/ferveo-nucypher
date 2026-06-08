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

