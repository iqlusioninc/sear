//! `sear` library error types

// TODO(tarcieri): use error macros
#![allow(unused_macros)]

use crate::prelude::*;
use core::fmt::{self, Display};
use failure::{Backtrace, Context, Fail};
#[cfg(feature = "std")]
use std::io;

/// Anything that can go wrong in sear
#[derive(Debug)]
pub struct Error {
    /// Contextual information about the error
    inner: Context<ErrorKind>,

    /// Optional description message
    description: Option<String>,
}

impl Error {
    /// Create a new error
    pub fn new(kind: ErrorKind) -> Self {
        Self {
            inner: Context::new(kind),
            description: None,
        }
    }

    /// Create a new error with the given description
    pub fn with_description(kind: ErrorKind, description: String) -> Self {
        Self {
            inner: Context::new(kind),
            description: Some(description),
        }
    }

    /// Obtain the inner `ErrorKind` for this error
    #[allow(dead_code)]
    pub fn kind(&self) -> ErrorKind {
        *self.inner.get_context()
    }
}

impl Fail for Error {
    fn cause(&self) -> Option<&dyn Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Self::new(kind)
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(inner: Context<ErrorKind>) -> Self {
        Self {
            inner,
            description: None,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.description {
            Some(ref desc) => write!(f, "{}: {}", &self.inner, desc),
            None => Display::fmt(&self.inner, f),
        }
    }
}

/// Kinds of errors
#[derive(Copy, Clone, Eq, PartialEq, Debug, Fail)]
pub enum ErrorKind {
    /// Invalid argument or other parameter
    #[fail(display = "invalid value")]
    Argument,

    /// Failure in a cryptographic primitive
    #[fail(display = "cryptographic failure")]
    Crypto,

    /// Input/output error
    #[cfg(feature = "std")]
    #[fail(display = "I/O error")]
    Io,

    /// Value overflow error
    #[fail(display = "value overflowed")]
    Overflow,

    /// Error parsing data
    #[fail(display = "parse error")]
    Parse,
}

/// Create a new error (of a given enum variant) with a formatted message
macro_rules! err {
    ($variant:ident, $msg:expr) => {
        $crate::error::Error::with_description(
            $crate::error::ErrorKind::$variant,
            $msg.to_string()
        )
    };
    ($variant:ident, $fmt:expr, $($arg:tt)+) => {
        $crate::error::Error::with_description(
            $crate::error::ErrorKind::$variant,
            format!($fmt, $($arg)+)
        )
    };
}

/// Create and return an error enum variant with a formatted message
macro_rules! fail {
    ($variant:ident, $msg:expr) => {
        return Err(err!($variant, $msg));
    };
    ($variant:ident, $fmt:expr, $($arg:tt)+) => {
        return Err(err!($variant, $fmt, $($arg)+));
    };
}

#[cfg(feature = "std")]
impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self {
        err!(Io, "{}", other)
    }
}
