//! sear: signed/encrypted archiver

#![deny(warnings, missing_docs, unused_import_braces, unused_qualifications)]
#![forbid(unsafe_code)]

#[macro_use]
pub mod error;

/// Archive builder
pub mod builder;

/// Keyring for encryption, signing, and verification keys
pub mod keyring;

/// Protocol Buffers which describe the structure of sear's archive format
pub mod protos;

pub use builder::Builder;
pub use error::Error;
pub use protos::entry::Entry;
