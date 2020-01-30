//! Archiving operations (parsed from command-line arguments)

mod chdir;
mod create;

use self::create::CreateOp;
use crate::{
    command::SearCmd,
    error::{Error, ErrorKind},
    prelude::*,
};
use abscissa_core::Runnable;
use std::convert::TryFrom;
use std::process::exit;

/// Operations on `.sear` files parsed from command-line arguments
#[derive(Debug)]
pub enum Op {
    /// Create a new `.sear` file
    Create(CreateOp),
}

impl TryFrom<&SearCmd> for Op {
    type Error = Error;

    /// Parse command-line arguments into the appropriate operation
    fn try_from(cmd: &SearCmd) -> Result<Self, Error> {
        if cmd.create && cmd.extract {
            fail!(ErrorKind::Argument, "-c and -x are orthogonal (pick one)");
        }

        if !cmd.create {
            fail!(ErrorKind::Argument, "neither -c nor -x specified");
        }

        Ok(Op::Create(CreateOp::new(cmd)?))
    }
}

impl Runnable for Op {
    fn run(&self) {
        let result = match self {
            Op::Create(create_op) => create_op.perform(),
        };

        if let Err(e) = result {
            status_err!("{}", e);
            exit(1);
        }
    }
}
