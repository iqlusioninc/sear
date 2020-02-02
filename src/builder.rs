//! `.sear` archive builder

use crate::{
    crypto::{
        stream::{self, writer::ChunkSize},
        symmetric,
    },
    entry::Entry,
    error::{Error, ErrorKind},
    protos::{Footer, Header, Index, MessageExt, Metadata, Tai64n},
    uuid,
};
use anomaly::{ensure, fail};
use std::{convert::TryInto, io};

/// File signature found at the beginning of sear archives which identifies
/// the format.
///
/// Version identifier which allows parsers to determine compatibility
/// and is incremented on breaking changes.
///
/// `0` indicates the format is unstable and archives with this number may or
/// may not be compatible with other `0`-versioned tooling.
pub const MAGIC_BYTES: &[u8; 6] = b"sear:0";

/// Archive builder
pub struct Builder<W: io::Write> {
    /// Entries within the archive
    entries: Vec<Entry>,

    /// Encrypted stream writer which is outputting the archive
    writer: stream::Writer<W>,
}

impl<W: io::Write> Builder<W> {
    /// Create a new archive builder wrapping the given writer
    pub fn new(mut writer: W, key: symmetric::Key, chunk_size: ChunkSize) -> Result<Self, Error> {
        // Write 6-byte sear archive magic
        writer.write_all(MAGIC_BYTES)?;

        // Generate random UUID identifying this archive
        let uuid = uuid::new_v4().to_string();

        let header = Header {
            uuid: pad_with_newlines(&uuid),
            chunk_size: chunk_size as u64,
            encryption_key_fingerprint: "".to_owned(),
            signing_key_fingerprint: "".to_owned(),
        }
        .to_vec()?;

        ensure!(
            header.len() <= std::u16::MAX as usize,
            ErrorKind::Overflow,
            "oversized header: {}-bytes",
            header.len()
        );

        // Write `u16` header length in little endian
        writer.write_all(&(header.len() as u16).to_le_bytes())?;

        // Write serialized `sear.header.Header` proto
        writer.write_all(&header)?;

        let stream_writer =
            stream::Writer::new(writer, key, uuid.as_bytes(), compute_aad(), chunk_size);

        Ok(Self {
            entries: vec![],
            writer: stream_writer,
        })
    }

    /// Append an entry to the archive
    pub fn append(&mut self, mut entry: Entry, reader: impl io::Read) -> Result<(), Error> {
        let length = self.writer.encrypt_reader(reader)? as u64;

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

    /// Finish writing the archive, adding the index and footer
    pub fn finish(mut self) -> Result<(), Error> {
        let mut index = Index { entries: vec![] };

        for entry in self.entries.into_iter() {
            index.entries.push(entry.try_into()?);
        }

        // TODO(tarcieri): support metadata located in the header instead of the footer
        let metadata = Metadata {
            index: Some(index),
            created_at: Some(Tai64n::now()),
            username: "".to_owned(),
            host: "".to_owned(),
        }
        .to_vec()?;

        self.writer.write_all(&metadata)?;

        // Finish writing the encrypted part of the stream, obtaining the
        // inner I/O object in order to write the plaintext footer
        let mut writer = self.writer.finish()?;

        let footer = Footer {
            metadata_length: metadata.len() as u64,
            signature: "".to_owned(), // TODO(tarcieri): signature support
        }
        .to_vec()?;

        writer.write_all(&footer)?;

        ensure!(
            footer.len() <= std::u16::MAX as usize,
            ErrorKind::Overflow,
            "oversized footer: {}-bytes",
            footer.len()
        );

        // Write `u16` footer length in little endian
        writer.write_all(&(footer.len() as u16).to_le_bytes())?;
        writer.flush()?;
        Ok(())
    }
}

/// Compute Additional Authenticated Data (AAD) to use when encrypting blocks
// TODO(tarcieri): include more data in this e.g. signature key
fn compute_aad() -> Vec<u8> {
    MAGIC_BYTES.to_vec()
}

/// Add leading and trailing newlines to an identifier (UUID or CryptoURI).
///
/// This makes these identifiers easier to manually inspect in archives.
fn pad_with_newlines(identifier: &str) -> String {
    format!("\n{}\n", identifier)
}
