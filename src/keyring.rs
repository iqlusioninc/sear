//! Key ring for encryption, signing, and verification keys

use crate::{
    crypto::kdf,
    error::{Error, ErrorKind},
};
use anomaly::format_err;
use std::{fs, io, path::Path};
use zeroize::Zeroizing;

/// Key ring
#[derive(Debug, Default)]
pub struct KeyRing {
    symmetric_keys: Vec<kdf::Key>,
}

impl KeyRing {
    /// Create an empty keyring
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a symmetric key to the keyring
    pub fn add_symmetric_key(&mut self, key: kdf::Key) {
        self.symmetric_keys.push(key);
    }

    /// Load a symmetric key stored on disk in CryptoURI format
    pub fn load_symmetric_key(&mut self, path: impl AsRef<Path>) -> Result<(), Error> {
        let path = path.as_ref();

        let key_uri = Zeroizing::new(fs::read_to_string(path).map_err(|e| {
            if e.kind() == io::ErrorKind::NotFound {
                format_err!(ErrorKind::FileNotFound, "{}", path.display()).into()
            } else {
                Error::from(e)
            }
        })?);

        let symmetric_key = kdf::Key::parse_uri(key_uri.trim_end()).map_err(|e| {
            format_err!(
                ErrorKind::Parse,
                "error loading key from {}: {}",
                path.display(),
                e
            )
        })?;

        self.add_symmetric_key(symmetric_key);
        Ok(())
    }

    /// Return the currently active encryption key if one is available
    pub fn symmetric_key(&self) -> Option<&kdf::Key> {
        // TODO(tarcieri): support for more than one key in the keyring
        match self.symmetric_keys.len() {
            0 => None,
            1 => self.symmetric_keys.get(0),
            _ => panic!("only one symmetric key per keyring presently supported"),
        }
    }
}
