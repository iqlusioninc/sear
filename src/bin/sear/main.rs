//! sear: signed/encrypted archive command line utility

#![forbid(unsafe_code)]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

mod application;
mod command;
mod config;
mod error;
mod formatter;
mod op;
mod prelude;

use application::APPLICATION;

fn main() {
    abscissa_core::boot(&APPLICATION);
}
