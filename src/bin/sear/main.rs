//! sear: signed/encrypted archive command line utility

#![deny(
    warnings,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_qualifications
)]
#![forbid(unsafe_code)]

mod application;
mod commands;
mod config;
mod error;
mod prelude;

use application::APPLICATION;

fn main() {
    abscissa::boot(&APPLICATION);
}
