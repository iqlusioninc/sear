//! Segmented AEAD streams

pub mod encryptor;
pub mod writer;

pub use self::{encryptor::Encryptor, writer::Writer};
