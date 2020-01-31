//! TAI64N timestamps

use chrono::{DateTime, Utc};
use std::time::SystemTime;

include!(concat!(env!("OUT_DIR"), "/sear.timestamp.rs"));

impl From<SystemTime> for Tai64n {
    fn from(time: SystemTime) -> Self {
        Self {
            value: tai64::TAI64N::from(time).to_bytes().as_ref().into(),
        }
    }
}

impl From<DateTime<Utc>> for Tai64n {
    fn from(time: DateTime<Utc>) -> Self {
        Self {
            value: tai64::TAI64N::from(time).to_bytes().as_ref().into(),
        }
    }
}

impl Tai64n {
    /// Create a `Tai64n` proto representing the current time
    pub fn now() -> Self {
        Self::from(SystemTime::now())
    }

    /// Create a Tai64n proto from a `std::time::SystemTime`
    pub fn from_system_time(time: &SystemTime) -> Self {
        Self::from(*time)
    }

    /// Convert a proto into a `std::time::SystemTime`
    ///
    /// Returns `Some` if the proto parses successfully as a `SystemTime`,
    /// or `None` otherwise
    pub fn to_system_time(&self) -> Option<SystemTime> {
        tai64::TAI64N::from_slice(&self.value)
            .map(|t| t.to_system_time())
            .ok()
    }

    /// Create a Tai64n proto from a `chrono::DateTime<Utc>`
    pub fn from_datetime_utc(time: &DateTime<Utc>) -> Self {
        Self::from(*time)
    }

    /// Convert a proto into a `std::time::SystemTime`
    ///
    /// Returns `Some` if the proto parses successfully as a `SystemTime`,
    /// or `None` otherwise
    pub fn to_datetime_utc(&self) -> Option<DateTime<Utc>> {
        tai64::TAI64N::from_slice(&self.value)
            .map(|t| t.to_datetime_utc())
            .ok()
    }
}
