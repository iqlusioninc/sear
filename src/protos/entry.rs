//! Entries (e.g. files) in `.sear` archives

#![allow(clippy::module_inception)]

include!(concat!(env!("OUT_DIR"), "/sear.entry.rs"));

pub use self::entry::{Kind, Owner};
