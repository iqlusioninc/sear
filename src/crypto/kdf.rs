//! Key derivation functionality

use super::symmetric;
use crate::{
    builder::MAGIC_BYTES,
    error::{Error, ErrorKind},
};
use aead::{generic_array::GenericArray, NewAead};
use aes_gcm::Aes256Gcm;
use anomaly::{fail, format_err};
use chacha20poly1305::ChaCha20Poly1305;
use cryptouri::{
    secret_key::{self, ExposeSecret, HkdfSha256Key},
    CryptoUri,
};
use hkdf::Hkdf;
use sha2::Sha256;
use std::fmt::{self, Debug};

/// Key derivation algorithm configured with input key material
// TODO(tarcieri): support for other KDFs besides HKDF-SHA-256?
pub struct Key(HkdfSha256Key);

impl Key {
    /// Parse an HKDF key from a CryptoURI
    pub fn parse_uri(key_str: &str) -> Result<Self, Error> {
        let key_uri = CryptoUri::parse_uri(key_str.trim_end())
            .map_err(|_e| format_err!(ErrorKind::Parse, "invalid CryptoURI"))?;

        let secret_key = key_uri
            .secret_key()
            .ok_or_else(|| format_err!(ErrorKind::Parse, "expected a crypto:sec:key"))?;

        let hkdf_key = secret_key
            .hkdfsha256_key()
            .ok_or_else(|| format_err!(ErrorKind::Parse, "expected a crypto:sec:key:hkdfsha256"))?;

        match hkdf_key.derived_alg() {
            Some(secret_key::Algorithm::Aes256Gcm)
            | Some(secret_key::Algorithm::ChaCha20Poly1305) => (),
            _ => fail!(
                ErrorKind::Parse,
                "key type must be hkdfsha256+aes256gcm or hkdfsha256+chacha20poly1305"
            ),
        }

        Ok(Key(hkdf_key.clone()))
    }

    /// Derive a symmetric key using HKDF
    pub fn derive_symmetric_key(&self, salt: impl AsRef<[u8]>) -> symmetric::Key {
        let mut derived_key = GenericArray::default();
        Hkdf::<Sha256>::new(Some(salt.as_ref()), self.0.expose_secret())
            .expand(MAGIC_BYTES, &mut derived_key)
            .expect("HKDF expand failed!");

        match self.0.derived_alg() {
            Some(secret_key::Algorithm::Aes256Gcm) => {
                symmetric::Key::Aes256Gcm(Box::new(Aes256Gcm::new(&derived_key)))
            }
            Some(secret_key::Algorithm::ChaCha20Poly1305) => {
                symmetric::Key::ChaCha20Poly1305(Box::new(ChaCha20Poly1305::new(&derived_key)))
            }
            _ => unreachable!(), // Checked above in `Key::parse_uri`
        }
    }
}

impl Debug for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        writeln!(
            f,
            "kdf::Key {{ derived_alg: {}, .. }}",
            self.0.derived_alg().unwrap()
        )
    }
}
