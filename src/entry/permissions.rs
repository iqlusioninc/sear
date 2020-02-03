//! File permissions

use crate::{
    error::{Error, ErrorKind},
    protos,
};
use anomaly::ensure;
use std::{
    convert::TryFrom,
    fmt::{self, Display},
    path::Path,
    str::FromStr,
};

#[cfg(unix)]
use std::{fs, os::unix::fs::MetadataExt};

/// File permissions
#[derive(Clone, Debug, PartialEq)]
pub struct Permissions {
    /// UNIX mode
    pub mode: Mode,

    /// POSIX ACLs
    pub posix_acls: Vec<PosixAcl>,

    /// SELinux file labels
    pub selinux_labels: Vec<SELinuxLabel>,
}

impl Permissions {
    /// Get the attributes for a file on the local filesystem
    #[cfg(unix)]
    pub fn for_file(path: &impl AsRef<Path>) -> Result<Self, Error> {
        let mode = Mode::try_from(fs::symlink_metadata(path)?.mode())?;

        // TODO(tarcieri): POSIX ACL support
        let posix_acls = vec![];

        // TODO(tarcieri): SELinux label support
        let selinux_labels = vec![];

        Ok(Permissions {
            mode,
            posix_acls,
            selinux_labels,
        })
    }

    /// Get the attributes for a file on the local filesystem
    #[cfg(windows)]
    pub fn for_file(_path: &impl AsRef<Path>) -> Result<Self, Error> {
        Ok(Permissions {
            // Use a default Unix mode value on Windows
            mode: Mode(0o644),
            posix_acls: vec![],
            selinux_labels: vec![],
        })
    }
}

impl From<Permissions> for protos::entry::Permissions {
    fn from(attrs: Permissions) -> Self {
        Self {
            mode: attrs.mode.into(),
            posix_acls: attrs.posix_acls.iter().map(ToString::to_string).collect(),
            selinux_labels: attrs
                .selinux_labels
                .iter()
                .map(ToString::to_string)
                .collect(),
        }
    }
}

/// Unix file mode (i.e. permissions)
///
/// See the `chmod(1)` manpage for more information.
// TODO(tarcieri): use a bitfield for representing this?
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Mode(u32);

impl Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:o}", self)
    }
}

impl fmt::Octal for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Octal::fmt(&self.0, f)
    }
}

impl From<Mode> for u32 {
    fn from(mode: Mode) -> u32 {
        mode.0
    }
}

impl TryFrom<u32> for Mode {
    type Error = Error;

    fn try_from(mode: u32) -> Result<Self, Error> {
        // TODO(tarcieri): validate mode
        Ok(Mode(mode))
    }
}

/// POSIX Access Control Lists (ACLs)
///
/// See the `setfacl(1)` manpage for more information.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PosixAcl(String);

impl AsRef<str> for PosixAcl {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Display for PosixAcl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_ref())
    }
}

impl FromStr for PosixAcl {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        ensure!(
            !s.is_empty(),
            ErrorKind::Parse,
            "POSIX ACLs cannot be empty"
        );

        // TODO(tarcieri): validate POSIX ACLs are well-structured
        Ok(PosixAcl(s.to_owned()))
    }
}

/// SELinux file labels: define the SELinux context of a file.
///
/// All SELinux policy decisions are based on these labels.
///
/// See the `semanage-fcontext(8)` manpage for more information.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SELinuxLabel(String);

impl AsRef<str> for SELinuxLabel {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Display for SELinuxLabel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_ref())
    }
}

impl FromStr for SELinuxLabel {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        ensure!(
            !s.is_empty(),
            ErrorKind::Parse,
            "SELinux labels cannot be empty"
        );

        // TODO(tarcieri): validate SELinux file labels are well-structured
        Ok(SELinuxLabel(s.to_owned()))
    }
}
