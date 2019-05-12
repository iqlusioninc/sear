/// Enum over the various key types
mod key;

/// Symmetric encryption keys
mod symmetric;

use self::key::Key;

/// Keyring for encryption, signing, and verification keys
pub struct Keyring {
    keys: Vec<Key>,
}

impl Keyring {
    /// Create an empty keyring
    pub fn new() -> Self {
        Self { keys: vec![] }
    }
}

impl Default for Keyring {
    fn default() -> Self {
        Self::new()
    }
}
