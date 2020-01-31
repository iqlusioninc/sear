//! UUIDs uniquely identify `sear` archives and function as an input to
//! HKDF to produce archive-specific cryptographic keys.

pub use uuid::Uuid;

use getrandom::getrandom;
use uuid::{Builder, Variant, Version};

/// Create a random UUID
pub fn new_v4() -> Uuid {
    // Get 128-bits of randomness from the OS's CSPRNG
    let mut bytes = uuid::Bytes::default();
    getrandom(&mut bytes).expect("RNG failure!");

    Builder::from_bytes(bytes)
        .set_variant(Variant::RFC4122)
        .set_version(Version::Random)
        .build()
}
