//! Sear Config

use abscissa_core::Config;
use serde::{Deserialize, Serialize};

/// Sear Configuration
#[derive(Clone, Config, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SearConfig {}
