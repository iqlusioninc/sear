//! Archiving operations (parsed from command-line arguments)

mod create;

use self::create::CreateOp;
use crate::command::SearCmd;
use abscissa_core::Runnable;
use failure::{bail, Error};
use std::convert::TryFrom;

/// Operations on `.sear` files parsed from command-line arguments
#[derive(Debug, Runnable)]
pub enum Operation {
    /// Create a new `.sear` file
    Create(CreateOp),
}

impl<'a> TryFrom<&'a SearCmd> for Operation {
    // TODO(tarcieri): use `crate::error::Error`
    type Error = Error;

    /// Parse command-line arguments into the appropriate operation
    fn try_from(cmd: &SearCmd) -> Result<Self, Error> {
        if cmd.create && cmd.extract {
            bail!("-c and -x are orthogonal (pick one)");
        }

        if !cmd.create {
            bail!("neither -c nor -x specified");
        }

        Ok(Operation::Create(CreateOp::new(cmd)?))
    }
}
