//! Segmented AEAD stream encryptor

use crate::{
    crypto::symmetric::{self, NONCE_SIZE},
    error::Error,
};
use aead::Buffer;

/// Size of a nonce prefix (nonce size less a 32-bit counter)
const NONCE_PREFIX_SIZE: usize = NONCE_SIZE - 4;

/// Index of the byte where we store the "last block" flag.
///
/// Unlike the original STREAM spec, we don't dedicate an entire byte to
/// this flag, but accept 7-bits of nonce data and only use one bit to store
/// the flag.
const LAST_BLOCK_FLAG_BYTE: usize = NONCE_PREFIX_SIZE - 1;

/// A STREAM encryptor with a 32-bit counter, generalized AEAD algorithms with
/// 96-bit nonces.
///
/// This corresponds to the ℰ stream encryptor object as defined in the paper
/// "Online Authenticated-Encryption and its Nonce-Reuse Misuse-Resistance".
///
/// <https://eprint.iacr.org/2015/189.pdf>
pub struct Encryptor {
    /// Encryption key
    key: symmetric::Key,

    /// Leading prefix of the STREAM nonce
    nonce_prefix: [u8; NONCE_PREFIX_SIZE],
}

impl Encryptor {
    /// Create a new STREAM encryptor, initialized with a given key and nonce.
    ///
    /// Panics if the key or nonce is the wrong size.
    pub fn new(key: symmetric::Key, mut nonce_prefix: [u8; NONCE_PREFIX_SIZE]) -> Self {
        // We store the "last block flag" in the last byte of the nonce prefix.
        // This clears the bit from the nonce prefix.
        nonce_prefix[LAST_BLOCK_FLAG_BYTE] &= 0xFE;

        Self { key, nonce_prefix }
    }

    /// Encrypt a message located at the given position in the stream in-place
    pub fn encrypt_in_place(
        &self,
        counter: u32,
        last_block: bool,
        associated_data: &[u8],
        buffer: &mut impl Buffer,
    ) -> Result<(), Error> {
        let nonce = self.stream_nonce(counter, last_block);
        self.key.encrypt_in_place(&nonce, associated_data, buffer)
    }

    /// Compute a STREAM nonce based on the given prefix, block counter, and
    /// last block flag.
    fn stream_nonce(&self, counter: u32, last_block: bool) -> [u8; NONCE_SIZE] {
        let mut nonce = [0u8; NONCE_SIZE];
        nonce[..NONCE_PREFIX_SIZE].copy_from_slice(&self.nonce_prefix);
        nonce[NONCE_PREFIX_SIZE..].copy_from_slice(&counter.to_le_bytes());

        if last_block {
            nonce[LAST_BLOCK_FLAG_BYTE] |= 1;
        }

        nonce
    }
}
