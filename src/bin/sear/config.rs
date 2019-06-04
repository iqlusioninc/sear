//! Sear Config
//!
//! See instructions in `commands.rs` to specify the path to your
//! application's configuration file and/or command-line options
//! for specifying it.

use abscissa::Config;
use serde::{Deserialize, Serialize};

/// Sear Configuration
#[derive(Clone, Config, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SearConfig {
    /// An example configuration section
    pub example_section: ExampleSection,
}

/// Example configuration section
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ExampleSection {
    /// Example configuration value
    pub example_value: String,
}
