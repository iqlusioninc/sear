//! Segmented AEAD STREAM writer

use crate::{
    crypto::{stream, symmetric},
    error::{Error, ErrorKind},
};
use anomaly::format_err;
use std::io;

/// Valid chunk sizes
#[repr(usize)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum ChunkSize {
    /// 1 kibibyte (1024 bytes)
    Kib1 = 1024,

    /// 128 kibibytes (131,072 bytes)
    Kib128 = 131_072,
}

impl Default for ChunkSize {
    fn default() -> Self {
        ChunkSize::Kib128
    }
}

/// Segmented AEAD STREAM writer
pub struct Writer<W: io::Write> {
    /// Internal buffer to fill before writing a chunk
    buffer: Vec<u8>,

    /// Position within our internal buffer
    buffer_pos: usize,

    /// Chunk counter
    chunk_counter: u32,

    /// Size of chunks to write
    chunk_size: ChunkSize,

    /// STREAM encryptor
    encryptor: stream::Encryptor,

    /// Underlying I/O object to write to
    io: W,
}

impl<W: io::Write> Writer<W> {
    /// Create a new STREAM writer
    pub fn new(io: W, key: symmetric::Key, chunk_size: ChunkSize) -> Self {
        // TODO(tarcieri): randomize nonce prefix?
        let nonce_prefix = Default::default();

        // Allocate the buffer with the chunk length + tag overhead,
        // i.e. the chunk size is the plaintext size NOT including the tag
        let mut buffer = vec![0u8; chunk_size as usize + symmetric::TAG_SIZE];

        // Truncate the tag size from the buffer (will be added on encryption)
        buffer.truncate(chunk_size as usize);

        Self {
            buffer,
            buffer_pos: 0,
            chunk_counter: 0,
            chunk_size,
            encryptor: stream::Encryptor::new(key, nonce_prefix),
            io,
        }
    }

    /// Encrypt the given input, filling the internal buffer and then
    /// encrypting a fixed-sized chunk using our STREAM writer
    pub fn encrypt_from_reader(&mut self, mut reader: impl io::Read) -> Result<usize, Error> {
        // Compute the total length of the input as we go
        let mut length = 0;

        loop {
            if self.buffer_pos == self.chunk_size as usize {
                self.encrypt_chunk()?;
            }

            let nbytes = reader.read(&mut self.buffer[self.buffer_pos..])?;
            length += nbytes;

            if nbytes == 0 {
                break;
            }
        }

        Ok(length)
    }

    /// Encrypt a chunk currently in the buffer, then clear the buffer
    fn encrypt_chunk(&mut self) -> Result<(), Error> {
        debug_assert_eq!(
            self.buffer_pos, self.chunk_size as usize,
            "buffer not full!"
        );

        // TODO(tarcieri): support for customizing AAD?
        let aad = b"";

        self.encryptor
            .encrypt_in_place(self.chunk_counter, false, aad, &mut self.buffer)?;

        self.chunk_counter = self
            .chunk_counter
            .checked_add(1)
            .ok_or_else(|| format_err!(ErrorKind::Crypto, "STREAM chunk counter overflowed"))?;

        self.io.write_all(&self.buffer)?;

        // Remove the MAC tag from the end of the buffer
        self.buffer.truncate(self.chunk_size as usize);
        self.buffer_pos = 0;

        Ok(())
    }
}
