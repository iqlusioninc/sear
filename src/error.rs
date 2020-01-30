//! `sear` library error types

use anomaly::{BoxError, Context};
use core::{
    fmt::{self, Display},
    ops::Deref,
};
use std::io;
use thiserror::Error;

/// Kinds of errors
#[derive(Copy, Clone, Debug, Eq, Error, PartialEq)]
pub enum ErrorKind {
    /// Invalid argument or other parameter
    #[error("invalid argument")]
    Argument,

    /// Failure in a cryptographic primitive
    #[error("crypto failure")]
    Crypto,

    /// File not found at the provided path
    #[error("file not found")]
    FileNotFound,

    /// Input/output error
    #[error("I/O error")]
    Io,

    /// Value overflow error
    #[error("value overflowed")]
    Overflow,

    /// Error parsing data
    #[error("parse error")]
    Parse,
}

impl ErrorKind {
    /// Create an error context from this error
    pub fn context(self, source: impl Into<BoxError>) -> Context<ErrorKind> {
        Context::new(self, Some(source.into()))
    }
}

/// Anything that can go wrong when `sear`-ing
#[derive(Debug)]
pub struct Error(Box<Context<ErrorKind>>);

impl Deref for Error {
    type Target = Context<ErrorKind>;

    fn deref(&self) -> &Context<ErrorKind> {
        &self.0
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Context::new(kind, None).into()
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(context: Context<ErrorKind>) -> Self {
        Error(Box::new(context))
    }
}

impl std::error::Error for Error {}

impl From<cryptouri::Error> for Error {
    fn from(err: cryptouri::error::Error) -> Error {
        ErrorKind::Parse.context(err).into()
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        ErrorKind::Io.context(err).into()
    }
}
