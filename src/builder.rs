//! Archive builder

use crate::{
    crypto::{stream, symmetric},
    error::{Error, ErrorKind},
    protos::entry::Entry,
};
use anomaly::fail;
use std::io;

/// Archive builder
pub struct Builder<W: io::Write> {
    /// Entries within the archive
    entries: Vec<Entry>,

    /// Encrypted stream writer which is outputting the archive
    writer: stream::Writer<W>,
}

impl<W: io::Write> Builder<W> {
    /// Create a new archive builder wrapping the given writer
    pub fn new(writer: W, key: symmetric::Key) -> Self {
        let entries = vec![];

        // TODO: configurable chunk size
        let chunk_size = stream::writer::ChunkSize::default();
        let writer = stream::Writer::new(writer, key, chunk_size);

        Self { entries, writer }
    }

    /// Append an entry to the archive
    pub fn append(&mut self, mut entry: Entry, reader: impl io::Read) -> Result<(), Error> {
        let length = self.writer.encrypt_from_reader(reader)? as u64;

        // Attempt to automatically set the length (protobufs defaults to 0 for missing fields)
        if entry.length != length {
            if entry.length == 0 {
                entry.length = length;
            } else {
                fail!(
                    ErrorKind::Argument,
                    "provided entry length ({}) does not match actual: {} bytes",
                    entry.length,
                    length
                );
            }
        }

        self.entries.push(entry);
        Ok(())
    }

    // TODO(tarcieri): footer support
}
