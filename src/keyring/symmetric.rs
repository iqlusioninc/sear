use crate::crypto::symmetric::{SealingKey, AES_128_GCM, AES_256_GCM};
use zeroize::Zeroize;

// Import these for convenience
use self::SymmetricKey::{Aes128Gcm, Aes256Gcm};

/// Types of symmetric keys (i.e. ciphers) supported by sear
pub enum SymmetricKey {
    /// AES-128 in Galois Counter Mode (GCM)
    Aes128Gcm([u8; 16]),

    /// AES-256 in Galois Counter Mode (GCM)
    Aes256Gcm([u8; 32]),
}

impl SymmetricKey {
    /// Instantiate a `SealingKey` from this symmetric key
    pub(crate) fn sealing_key(&self) -> SealingKey {
        let algorithm = match self {
            Aes128Gcm(_) => &AES_128_GCM,
            Aes256Gcm(_) => &AES_256_GCM,
        };

        SealingKey::new(
            algorithm,
            match self {
                Aes128Gcm(ref key) => key,
                Aes256Gcm(ref key) => key,
            },
        )
        .unwrap()
    }
}

/// Use `clear_on_drop` to clear the key from memory
impl Drop for SymmetricKey {
    fn drop(&mut self) {
        match *self {
            Aes128Gcm(ref mut key) => key.zeroize(),
            Aes256Gcm(ref mut key) => key.zeroize(),
        }
    }
}
