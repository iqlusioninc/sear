use failure::Error;
use std::env;
use std::path::PathBuf;

/// Support for optionally changing the current directory prior to performing
/// an operation
#[derive(Debug)]
pub struct Chdir {
    path: Option<PathBuf>,
}

impl Chdir {
    /// Create a new `Chdir` struct with an optional `PathBuf`
    pub fn new(path: Option<PathBuf>) -> Self {
        Self { path }
    }

    /// Perform the directory change operation if we're configured to
    pub fn perform(&self) -> Result<(), Error> {
        if let Some(ref path) = self.path {
            env::set_current_dir(path)?;
        }

        Ok(())
    }
}
