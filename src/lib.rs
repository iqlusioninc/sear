//! `sear`: Signed / Encrypted ARchive
//!
//! This module contains the library components of `sear`.
//!
//! The command-line utility functionality is contained within the
//! `src/bin/sear` subdirectory.

#![doc(html_root_url = "https://docs.rs/sear/0.0.0")]
#![forbid(unsafe_code)]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

#[macro_use]
pub mod error;

pub mod builder;
pub mod crypto;
pub mod entry;
pub mod keyring;
pub mod protos;
pub mod uuid;

pub use self::{builder::Builder, entry::Entry, error::Error, keyring::KeyRing};
