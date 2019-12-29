//! Create a new `.sear` archive

use crate::{
    command::SearCmd,
    error::{Error, ErrorKind},
    prelude::*,
};
use abscissa_core::Runnable;
use std::path::PathBuf;

/// Create a new `.sear` archive
#[derive(Debug)]
pub struct CreateOp {
    /// Input/output archive file
    pub archive: PathBuf,

    /// Preserve absolute pathnames
    pub preserve_pathnames: bool,

    /// Preserve file permissions
    pub preserve_permissions: bool,

    /// Verbose mode
    pub verbose: bool,

    /// Files to include in the archive
    pub files: Vec<PathBuf>,
}

impl CreateOp {
    /// Initialize a create operation from command-line arguments
    pub fn new(cmd: &SearCmd) -> Result<Self, Error> {
        let archive = match cmd.archive {
            Some(ref path) => PathBuf::from(path),
            None => fail!(ErrorKind::Argument, "no -f option given"),
        };

        let files = cmd.files.iter().map(PathBuf::from).collect();

        Ok(Self {
            archive,
            files,
            preserve_pathnames: cmd.preserve_pathnames,
            preserve_permissions: cmd.preserve_permissions,
            verbose: cmd.verbose,
        })
    }
}

impl Runnable for CreateOp {
    fn run(&self) {
        unimplemented!();
    }
}
