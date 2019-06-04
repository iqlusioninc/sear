//! sear: CLI subcommands

mod start;
mod version;

use self::{start::StartCommand, version::VersionCommand};
use crate::config::SearConfig;
use abscissa::{Command, Configurable, Options, Runnable};
use std::path::PathBuf;

/// Sear Subcommands
#[derive(Command, Debug, Options, Runnable)]
pub enum SearCommand {
    /// The `start` subcommand
    #[options(help = "start the application")]
    Start(StartCommand),

    /// The `version` subcommand
    #[options(help = "display version information")]
    Version(VersionCommand),
}

/// This trait allows you to define how applicaiton configuration is loaded.
impl Configurable<SearConfig> for SearCommand {
    fn config_path(&self) -> Option<PathBuf> {
        // Have `config_path` return `Some(path)` in order to trigger the
        // application configuration being loaded.
        None
    }
}
