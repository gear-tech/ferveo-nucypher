#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod keypair;
pub mod serialization;

pub use keypair::*;
pub use serialization::*;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid byte length: expected {expected}, actual {actual}")]
    InvalidByteLength { expected: usize, actual: usize },
    #[error("Serialization error: {0}")]
    SerializationError(ark_serialize::SerializationError),
    #[error("Invalid seed length: {0}")]
    InvalidSeedLength(usize),
}

type Result<T> = core::result::Result<T, Error>;
