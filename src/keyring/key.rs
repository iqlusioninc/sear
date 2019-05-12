use super::symmetric::SymmetricKey;

/// Enum of symmetric encryption, signing, and verification keys
pub enum Key {
    /// Symmetric (i.e. AEAD) encryption key
    Symmetric(SymmetricKey),
}

impl Key {
    /// Return a reference to the symmetric key, if the enum variant is symmetric
    pub fn symmetric(&self) -> Option<&SymmetricKey> {
        match self {
            Key::Symmetric(ref key) => Some(key),
        }
    }

    /// Is this a symmetric key?
    pub fn is_symmetric(&self) -> bool {
        self.symmetric().is_some()
    }
}
