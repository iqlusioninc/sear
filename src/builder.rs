use crate::{
    crypto::symmetric::{SealingKey, StreamEncryptor},
    error::Error,
    protos::entry::Entry,
};
use std::io::{Read, Write};

/// Default chunk size (in bytes) to use for encrypted segments of an archive
///
/// For MACs with a 128-bit tag, this results in an encrypted chunk size of 16400 bytes.
pub const DEFAULT_CHUNK_SIZE: usize = 16384;

/// Archive builder
pub struct Builder<W: Write> {
    /// Entries within the archive
    entries: Vec<Entry>,

    /// Chunk size for encrypted message segments,
    chunk_size: usize,

    /// Streaming encryptor which is outputting the archive
    encryptor: StreamEncryptor<W>,
}

impl<W: Write> Builder<W> {
    /// Create a new archive builder wrapping the given writer
    pub fn new(writer: W, key: SealingKey) -> Self {
        let entries = vec![];

        // TODO: configurable chunk size
        let chunk_size = DEFAULT_CHUNK_SIZE;
        let encryptor = StreamEncryptor::new(writer, key, chunk_size);

        Self {
            entries,
            chunk_size,
            encryptor,
        }
    }

    /// Append an entry to the archive
    pub fn append<R: Read>(&mut self, mut entry: Entry, reader: R) -> Result<(), Error> {
        let length = self.encrypt_input(reader)?;

        // Attempt to automatically set the length (protobufs defaults to 0 for missing fields)
        if entry.length != length {
            if entry.length == 0 {
                entry.length = length;
            } else {
                fail!(
                    InvalidValue,
                    "provided entry length ({}) does not match actual: {} bytes",
                    entry.length,
                    length
                );
            }
        }

        self.entries.push(entry);

        panic!("unimplemented");
    }

    /// Encrypt an input file, writing it out to the encryptor
    fn encrypt_input<R: Read>(&mut self, mut reader: R) -> Result<u64, Error> {
        let mut length: usize = 0;
        let mut buffer = vec![0u8; self.chunk_size];

        loop {
            let nbytes = reader.read(&mut buffer)?;

            if nbytes == 0 {
                break;
            }

            self.encryptor.write(&buffer[..nbytes])?;
            length += nbytes;
        }

        Ok(length as u64)
    }
}
