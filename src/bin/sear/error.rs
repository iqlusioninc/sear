//! `sear` CLI application error types

use abscissa::{err, Error};
use failure::Fail;
use std::{fmt, io};

/// Error type
#[derive(Debug)]
pub struct SearError(Error<SearErrorKind>);

/// Kinds of errors
#[derive(Copy, Clone, Eq, PartialEq, Debug, Fail)]
pub enum SearErrorKind {
    /// Input/output error
    #[fail(display = "I/O error")]
    Io,
}

impl fmt::Display for SearError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl From<Error<SearErrorKind>> for SearError {
    fn from(other: Error<SearErrorKind>) -> Self {
        SearError(other)
    }
}

impl From<io::Error> for SearError {
    fn from(other: io::Error) -> Self {
        err!(SearErrorKind::Io, other).into()
    }
}
