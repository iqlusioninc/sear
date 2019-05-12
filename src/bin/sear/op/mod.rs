use failure::Error;
use gumdrop::Options;
use std::process::exit;

mod create;

use self::create::CreateOp;
use args::Args;

/// Operations on `.sear` files parsed from command-line arguments
#[derive(Debug)]
pub enum Op {
    /// Create a new `.sear` file
    Create(CreateOp),
}

impl Op {
    /// Parse Op from the command line arguments or exit
    pub fn parse_from_args_or_exit() -> Self {
        let args = Args::parse_args_default_or_exit();
        Self::from_args(args).unwrap_or_else(|_| exit(2))
    }

    /// Parse Op to perform from Op-line options
    pub fn from_args(args: Args) -> Result<Self, Error> {
        if args.create && args.extract {
            eprintln!("sear: Cannot specify both the '-c' and '-x' options (pick one)");
            eprintln!("Try 'sear --help' for more information");

            bail!("-c and -x are orthogonal");
        }

        if args.create {
            Ok(Op::Create(CreateOp::new(args)?))
        } else {
            eprintln!("sear: Must specify either the '-c' or '-x' option");
            eprintln!("Try 'sear --help' for more information");

            bail!("neither -c nor -x specified");
        }
    }

    /// Perform the given operation
    pub fn perform(&self) -> Result<(), Error> {
        match self {
            &Op::Create(ref op) => op.perform(),
        }
    }
}
