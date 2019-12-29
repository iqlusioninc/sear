//! `sear` library error types

use crate::prelude::*;
use core::fmt::{self, Display};
#[cfg(feature = "std")]
use std::io;
use thiserror::Error;

/// Create a new error (of a given enum variant) with a formatted message
macro_rules! format_err {
    ($variant:ident, $msg:expr) => {
        $crate::error::Error::with_description(
            $crate::error::ErrorKind::$variant,
            $msg.to_string()
        )
    };
    ($variant:ident, $fmt:expr, $($arg:tt)+) => {
        $crate::error::Error::new(
            $crate::error::ErrorKind::$variant,
            Some(format!($fmt, $($arg)+))
        )
    };
}

/// Create and return an error enum variant with a formatted message
// TODO(tarcieri): actually use this macro
#[allow(unused_macros)]
macro_rules! fail {
    ($variant:ident, $msg:expr) => {
        return Err(err!($variant, $msg));
    };
    ($variant:ident, $fmt:expr, $($arg:tt)+) => {
        return Err(err!($variant, $fmt, $($arg)+));
    };
}

/// Kinds of errors
#[derive(Copy, Clone, Debug, Eq, Error, PartialEq)]
pub enum ErrorKind {
    /// Invalid argument or other parameter
    #[error("invalid value")]
    Argument,

    /// Failure in a cryptographic primitive
    #[error("cryptographic failure")]
    Crypto,

    /// Input/output error
    #[cfg(feature = "std")]
    #[error("I/O error")]
    Io,

    /// Value overflow error
    #[error("value overflowed")]
    Overflow,

    /// Error parsing data
    #[error("parse error")]
    Parse,
}

/// Anything that can go wrong in sear
#[derive(Debug)]
pub struct Error {
    /// Contextual information about the error
    kind: ErrorKind,

    /// Optional description message
    msg: Option<String>,
}

impl Error {
    /// Create a new error
    pub fn new(kind: ErrorKind, msg: Option<String>) -> Self {
        Self { kind, msg }
    }

    /// Obtain the inner `ErrorKind` for this error
    #[allow(dead_code)]
    pub fn kind(&self) -> ErrorKind {
        self.kind
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.kind.fmt(f)
    }
}

impl std::error::Error for Error {}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Self::new(kind, None)
    }
}

#[cfg(feature = "std")]
impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self {
        format_err!(Io, "{}", other)
    }
}
