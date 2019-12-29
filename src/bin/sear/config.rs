//! Sear Config

use serde::{Deserialize, Serialize};

/// Sear Configuration
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SearConfig {}
