//! File owners

use crate::{
    error::{Error, ErrorKind},
    protos,
};
use anomaly::{ensure, fail};
use std::{
    fmt::{self, Display},
    fs::Metadata,
    str::FromStr,
};

/// File owner: user/group who owns a particular file
#[derive(Clone, Debug, PartialEq)]
pub enum Owner {
    /// Numerical IDs
    Id {
        /// User ID
        uid: u32,

        /// Group ID
        gid: u32,
    },

    /// Named owner
    Name {
        /// Name of the owning user
        username: Name,

        /// Name of the owning group
        groupname: Name,
    },

    /// Unspecified owner
    Unspecified,
}

#[cfg(unix)]
impl From<&Metadata> for Owner {
    fn from(metadata: &Metadata) -> Self {
        use std::os::unix::fs::MetadataExt;

        Owner::Id {
            uid: metadata.uid(),
            gid: metadata.gid(),
        }
    }
}

#[cfg(windows)]
impl From<&Metadata> for Owner {
    fn from(_metadata: &Metadata) -> Self {
        Owner::Unspecified
    }
}

impl From<Owner> for protos::entry::Owner {
    fn from(owner: Owner) -> Self {
        match owner {
            Owner::Id { uid, gid } => {
                let owner_id = protos::entry::OwnerId { uid, gid };
                protos::entry::Owner::Id(owner_id)
            }
            Owner::Name {
                username,
                groupname,
            } => {
                let owner_name = protos::entry::OwnerName {
                    username: username.to_string(),
                    groupname: groupname.to_string(),
                };
                protos::entry::Owner::Name(owner_name)
            }
            Owner::Unspecified => {
                protos::entry::Owner::Unspecified(protos::entry::OwnerUnspecified {})
            }
        }
    }
}

/// User or group names
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Name(String);

impl AsRef<str> for Name {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_ref())
    }
}

impl FromStr for Name {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        ensure!(!s.is_empty(), ErrorKind::Parse, "names cannot be empty");

        for (i, c) in s.chars().enumerate() {
            match c {
                'a'..='z' => continue,
                '0'..='9' | '-' => {
                    if i != 0 {
                        continue;
                    }
                }
                _ => (),
            }

            fail!(ErrorKind::Parse, "invalid char `{}` in name: `{}`", c, s);
        }

        Ok(Name(s.to_owned()))
    }
}
