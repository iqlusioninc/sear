//! Application-local prelude: conveniently import types/functions/macros
//! which are generally useful and should be available everywhere.

/// Commonly used Abscissa traits
pub use abscissa::{Application, Command, Runnable};

/// Logging macros
pub use abscissa::log::{debug, error, info, log, log_enabled, trace, warn};
