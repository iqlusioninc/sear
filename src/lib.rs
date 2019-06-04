//! `sear`: Signed / Encrypted ARchive
//!
//! This module contains the library components of `sear`.
//!
//! The command-line utility functionality is contained within the
//! `src/bin/sear` subdirectory.

#![no_std]
#![deny(
    warnings,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_qualifications
)]
#![forbid(unsafe_code)]

#[cfg(not(feature = "std"))]
compile_error!("no_std is not yet supported (will require alloc crate)");

#[cfg(feature = "std")]
#[macro_use]
extern crate std;

pub mod error;
pub mod prelude;
pub mod protos;
