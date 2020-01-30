//! Key ring for encryption, signing, and verification keys

use crate::{
    crypto::symmetric,
    error::{Error, ErrorKind},
};
use aead::{generic_array::GenericArray, NewAead};
use aes_gcm::Aes256Gcm;
use anomaly::{fail, format_err};
use chacha20poly1305::ChaCha20Poly1305;
use cryptouri::{
    secret_key::{ExposeSecret, SecretKey},
    CryptoUri,
};
use std::{fs, io, path::Path};
use zeroize::Zeroizing;

/// Key ring
#[derive(Debug, Default)]
pub struct KeyRing {
    symmetric_keys: Vec<symmetric::Key>,
}

impl KeyRing {
    /// Create an empty keyring
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a symmetric key to the keyring
    pub fn add_symmetric_key(&mut self, key: symmetric::Key) {
        self.symmetric_keys.push(key);
    }

    /// Load a symmetric key stored on disk in CryptoURI format
    pub fn load_symmetric_key(&mut self, path: impl AsRef<Path>) -> Result<(), Error> {
        let path = path.as_ref();

        let key_str = Zeroizing::new(fs::read_to_string(path).map_err(|e| {
            if e.kind() == io::ErrorKind::NotFound {
                format_err!(ErrorKind::FileNotFound, "{}", path.display()).into()
            } else {
                Error::from(e)
            }
        })?);

        let key_uri = CryptoUri::parse_uri(key_str.trim_end())?;

        let secret_key = key_uri
            .secret_key()
            .ok_or_else(|| format_err!(ErrorKind::Parse, "expected a crypto::sec::key"))?;

        let symmetric_key = match secret_key {
            SecretKey::Aes256Gcm(key) => {
                let key = GenericArray::clone_from_slice(key.expose_secret());
                symmetric::Key::Aes256Gcm(Box::new(Aes256Gcm::new(key)))
            }
            SecretKey::ChaCha20Poly1305(key) => {
                let key = GenericArray::clone_from_slice(key.expose_secret());
                symmetric::Key::ChaCha20Poly1305(Box::new(ChaCha20Poly1305::new(key)))
            }
            _ => fail!(ErrorKind::Parse, "expected a crypto::sec::key::aes256gcm"),
        };

        self.add_symmetric_key(symmetric_key);
        Ok(())
    }

    /// Return the currently active encryption key if one is available
    pub fn symmetric_key(&self) -> Option<&symmetric::Key> {
        // TODO(tarcieri): support for more than one key in the keyring
        match self.symmetric_keys.len() {
            0 => None,
            1 => self.symmetric_keys.get(0),
            _ => panic!("only one symmetric key per keyring presently supported"),
        }
    }
}
