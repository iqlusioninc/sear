//! sear: signed/encrypted archive command line utility

#![deny(
    warnings,
    missing_docs,
    unsafe_code,
    unused_import_braces,
    unused_qualifications
)]

#[macro_use]
extern crate failure;
#[macro_use]
extern crate gumdrop_derive;

use std::process::exit;

/// Command-line argument parsing
mod args;

/// Support for (optionally) changing the current directory prior to performing an op
mod chdir;

/// Operations (i.e. create vs extract)
mod op;

use op::Op;

/// Main entry point
fn main() {
    // Parse operation to perform from command-line arguments
    let op = Op::parse_from_args_or_exit();

    println!("op: {:?}", op);

    // Perform the operation
    op.perform().unwrap_or_else(|e| {
        eprintln!("error: {}", e);
        exit(1);
    });
}
