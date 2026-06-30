use alloc::vec::Vec;

use crate::Result;

/// Encoding contract for typed plaintext wrappers.
///
/// The byte-oriented encryption API remains the interoperability boundary.
/// Implement this trait for types that need typed encrypt/decrypt helpers.
pub trait Codec: Sized {
    fn encode(&self) -> Result<Vec<u8>>;

    fn decode(bytes: &[u8]) -> Result<Self>;
}

/// Delegating [Codec] implementation to [parity_scale_codec].
#[cfg(feature = "parity-codec")]
impl<T> Codec for T
where
    T: parity_scale_codec::Encode + parity_scale_codec::Decode,
{
    fn encode(&self) -> Result<Vec<u8>> {
        Ok(<T as parity_scale_codec::Encode>::encode(self))
    }

    fn decode(bytes: &[u8]) -> Result<Self> {
        Ok(<T as parity_scale_codec::Decode>::decode(&mut &bytes[..])?)
    }
}
