//! Protocol Buffers which describe the structure of sear's archive format

#![allow(missing_docs, unused_qualifications)]

pub mod entry;
pub mod footer;
pub mod header;
pub mod metadata;
pub mod timestamp;

pub use self::{
    entry::Entry,
    footer::Footer,
    header::Header,
    metadata::{Index, Metadata},
    timestamp::Tai64n,
};

use crate::error::Error;
use prost::Message;

/// Extension to `prost::Message` to simplify encoding to a `Vec<u8>`
pub trait MessageExt: Message + Sized {
    fn to_vec(&self) -> Result<Vec<u8>, Error> {
        let mut bytes = Vec::with_capacity(self.encoded_len());
        self.encode(&mut bytes)?;
        Ok(bytes)
    }
}

impl<M: Message> MessageExt for M {}
