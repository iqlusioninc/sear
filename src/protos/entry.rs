//! Entries (e.g. files) in archives

#![allow(clippy::module_inception)]

include!(concat!(env!("OUT_DIR"), "/sear.entry.rs"));

pub use self::entry::Kind;
