//! TAI64N timestamps

use chrono::{DateTime, Utc};
use std::time::SystemTime;

include!(concat!(env!("OUT_DIR"), "/sear.timestamp.rs"));

impl Tai64n {
    /// Create a `Tai64n` proto representing the current time
    pub fn now() -> Self {
        Self::from(SystemTime::now())
    }
}

impl From<DateTime<Utc>> for Tai64n {
    fn from(time: DateTime<Utc>) -> Self {
        tai64::TAI64N::from(time).into()
    }
}

impl From<SystemTime> for Tai64n {
    fn from(time: SystemTime) -> Self {
        tai64::TAI64N::from(time).into()
    }
}

impl From<tai64::TAI64N> for Tai64n {
    fn from(t: tai64::TAI64N) -> Self {
        Self {
            value: t.to_bytes().to_vec(),
        }
    }
}
