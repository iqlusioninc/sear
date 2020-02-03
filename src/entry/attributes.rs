//! File attributes

use crate::{
    error::{Error, ErrorKind},
    protos,
};
use anomaly::ensure;
use mime::Mime;
use std::{
    fmt::{self, Display},
    fs,
    path::Path,
    str::FromStr,
};
use tai64::TAI64N;

/// File attributes
#[derive(Clone, Debug, PartialEq)]
pub struct Attributes {
    /// Date when the file was created
    pub created_at: Option<TAI64N>,

    /// Date when the file was last modified
    pub modified_at: Option<TAI64N>,

    /// Media type (a.k.a. MIME type)
    pub content_type: Mime,

    /// Extended attributes
    pub xattr: Vec<XAttr>,
}

impl Attributes {
    /// Get the attributes for a file on the local filesystem
    pub fn for_file(path: &impl AsRef<Path>) -> Result<Self, Error> {
        let metadata = fs::symlink_metadata(path)?;

        // Tolerate platforms where creation time/modification time are unavailable
        let created_at = metadata.created().ok().map(Into::into);
        let modified_at = metadata.modified().ok().map(Into::into);

        let content_type = if metadata.file_type().is_symlink() {
            "inode/symlink".parse()?
        } else {
            // TODO(tarcieri): support for disabling automatic MIME type detection
            tree_magic::from_filepath(path.as_ref()).parse()?
        };

        // TODO(tarcieri): xattr support
        let xattr = vec![];

        Ok(Self {
            created_at,
            modified_at,
            content_type,
            xattr,
        })
    }
}

impl From<Attributes> for protos::entry::Attributes {
    fn from(attrs: Attributes) -> Self {
        Self {
            created_at: attrs.created_at.map(Into::into),
            modified_at: attrs.modified_at.map(Into::into),
            content_type: attrs.content_type.to_string(),
            xattr: attrs.xattr.iter().map(ToString::to_string).collect(),
        }
    }
}

/// Extended attributes for modern Unix filesystems.
///
/// See the `attr(5)` manpage for more information.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct XAttr(String);

impl AsRef<str> for XAttr {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Display for XAttr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_ref())
    }
}

impl FromStr for XAttr {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        ensure!(!s.is_empty(), ErrorKind::Parse, "xattrs cannot be empty");

        // TODO(tarcieri): validate xattrs are well-structured
        Ok(XAttr(s.to_owned()))
    }
}
