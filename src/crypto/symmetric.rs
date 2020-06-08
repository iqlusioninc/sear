//! Symmetric cryptography support

use crate::error::{Error, ErrorKind};
use aead::{generic_array::GenericArray, AeadInPlace, Buffer};
use aes_gcm::Aes256Gcm;
use chacha20poly1305::ChaCha20Poly1305;
use std::fmt::{self, Debug};

/// Total size of a nonce
pub const NONCE_SIZE: usize = 12;

/// Size of the authentication (i.e. MAC) tag added to every message
pub const TAG_SIZE: usize = 16;

/// Symmetric encryption keys
#[derive(Clone)]
pub enum Key {
    /// AES-256-GCM
    Aes256Gcm(Box<Aes256Gcm>),

    /// ChaCha20Poly1305
    ChaCha20Poly1305(Box<ChaCha20Poly1305>),
}

impl Key {
    /// Encrypt the given buffer containing a plaintext message in-place.
    pub fn encrypt_in_place(
        &self,
        nonce: &[u8; NONCE_SIZE],
        associated_data: &[u8],
        buffer: &mut impl Buffer,
    ) -> Result<(), Error> {
        let nonce = GenericArray::from(*nonce);

        match self {
            Key::Aes256Gcm(key) => key.encrypt_in_place(&nonce, associated_data, buffer),
            Key::ChaCha20Poly1305(key) => key.encrypt_in_place(&nonce, associated_data, buffer),
        }
        .map_err(|_| ErrorKind::Crypto.into())
    }
}

impl Debug for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.write_str(match self {
            Key::Aes256Gcm(_) => "symmetric::Key::Aes256Gcm {{ .. }}",
            Key::ChaCha20Poly1305(_) => "symmetric::Key::ChaCha20Poly1305 {{ .. }}",
        })
    }
}
