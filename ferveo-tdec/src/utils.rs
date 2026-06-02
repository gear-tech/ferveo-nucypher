pub mod ark_serde {
    use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
    use serde::{Deserialize, Serialize};

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: CanonicalSerialize,
        S: serde::Serializer,
    {
        let mut bytes = Vec::with_capacity(value.compressed_size());
        value
            .serialize_compressed(&mut bytes)
            .map_err(serde::ser::Error::custom)?;

        serde_bytes::ByteBuf::from(bytes).serialize(serializer)
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        T: CanonicalDeserialize,
        D: serde::Deserializer<'de>,
    {
        let bytes = serde_bytes::ByteBuf::deserialize(deserializer)?;
        T::deserialize_compressed(&mut bytes.as_slice())
            .map_err(serde::de::Error::custom)
    }
}
