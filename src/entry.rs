//! Entry (i.e. file) in a `.sear` archive
//!
//! These types are structurally similar to the ones defined in `entry.proto`,
//! but contain the actual domain logic for representing entries, leaving the
//! Protobuf-defined types exclusively for serialization.

pub mod attributes;
pub mod owner;
pub mod permissions;

pub use self::{attributes::Attributes, owner::Owner, permissions::Permissions};

use crate::{
    error::{Error, ErrorKind},
    protos,
};
use anomaly::{fail, format_err};
use std::{
    convert::{TryFrom, TryInto},
    fs,
    path::{Path, PathBuf},
};

/// Entry within a .sear archive file
#[derive(Clone, Debug, PartialEq)]
pub struct Entry {
    /// Path to the file (absolute or relative, with '.' and '..' disallowed)
    pub path: PathBuf,

    /// Length of the file in bytes
    pub length: u64,

    /// File owner
    pub owner: Owner,

    /// File permissions
    pub permissions: Permissions,

    /// File attributes
    pub attributes: Attributes,

    /// Kinds of entries
    pub kind: Kind,
}

impl Entry {
    /// Create an [`Entry`] for a file on the local filesystem
    pub fn for_file(path: impl AsRef<Path>) -> Result<Self, Error> {
        let path = path.as_ref().to_owned();

        if path.to_str().is_none() {
            fail!(
                ErrorKind::Path,
                "invalid UTF-8 in path: `{}`",
                path.display()
            );
        }

        let metadata = fs::symlink_metadata(&path)?;
        let length = metadata.len();
        let owner = Owner::from(&metadata);
        let kind = Kind::for_file(&path)?;
        let permissions = Permissions::for_file(&path)?;
        let attributes = Attributes::for_file(&path)?;

        Ok(Self {
            path,
            length,
            owner,
            permissions,
            attributes,
            kind,
        })
    }

    /// Format the file length in a human-friendly way: with length suffixes
    /// (e.g. KiB, MiB, GiB) ala the `-h` flag in `ls` and `df`
    pub fn length_formatted(&self) -> String {
        /// KiB
        const KIBIBYTE: f64 = 1024.0;

        /// MiB
        const MEBIBYTE: f64 = 1_048_576.0;

        /// GiB
        const GIBIBYTE: f64 = 1_073_741_824.0;

        let n = self.length;

        if n < KIBIBYTE as u64 {
            // bytes
            if n == 1 {
                return "1 byte".to_owned();
            } else {
                return format!("{} bytes", n);
            }
        }

        let n = n as f64;

        if n < MEBIBYTE {
            // kibibytes
            format!("{} KiB", n / KIBIBYTE)
        } else if n < GIBIBYTE {
            // mebibytes
            format!("{} MiB", n / MEBIBYTE)
        } else {
            // gibibytes
            format!("{} GiB", n / GIBIBYTE)
        }
    }
}

impl TryFrom<Entry> for protos::Entry {
    type Error = Error;

    fn try_from(entry: Entry) -> Result<Self, Error> {
        let path_str = entry.path.to_str().ok_or_else(|| {
            format_err!(
                ErrorKind::Path,
                "invalid UTF-8 in path: `{}`",
                entry.path.display()
            )
        })?;

        Ok(Self {
            path: path_str.to_owned(),
            length: entry.length,
            owner: Some(entry.owner.into()),
            permissions: Some(entry.permissions.into()),
            attributes: Some(entry.attributes.into()),
            kind: Some(entry.kind.try_into()?),
        })
    }
}

/// Kinds of entries
// TODO(tarcieri): character devices, block devices, directories, FIFOs, etc.
#[derive(Clone, Debug, PartialEq)]
pub enum Kind {
    /// Regular files
    File,

    /// Hard or symbolic links
    Link {
        /// Is this a symbolic (as opposed to hard) link?
        symbolic: bool,

        /// Target path the link is pointing to
        target: PathBuf,
    },
}

impl Kind {
    /// Get the kind of file on the local filesystem
    pub fn for_file(path: &impl AsRef<Path>) -> Result<Self, Error> {
        let file_type = fs::symlink_metadata(path)?.file_type();

        if file_type.is_symlink() {
            // TODO(tarcieri): hard links?
            let target = fs::read_link(path)?;
            Ok(Kind::Link {
                symbolic: true,
                target,
            })
        } else if file_type.is_file() {
            Ok(Kind::File)
        } else {
            // TODO(tarcieri): handle directories
            fail!(ErrorKind::Path, "directories unsupported")
        }
    }
}

impl TryFrom<Kind> for protos::entry::Kind {
    type Error = Error;

    fn try_from(kind: Kind) -> Result<Self, Error> {
        Ok(match kind {
            Kind::File => protos::entry::Kind::File(protos::entry::FileEntry {}),
            Kind::Link { symbolic, target } => {
                let target_str = target.to_str().ok_or_else(|| {
                    format_err!(
                        ErrorKind::Path,
                        "invalid UTF-8 in path: `{}`",
                        target.display()
                    )
                })?;

                protos::entry::Kind::Link(protos::entry::LinkEntry {
                    symbolic,
                    target: target_str.to_owned(),
                })
            }
        })
    }
}
