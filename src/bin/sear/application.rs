//! `sear`: CLI application (built on the Abscissa framework)

use crate::{command::SearCommand, config::SearConfig};
use abscissa::{application, Application, FrameworkError, LoggingConfig, StandardPaths};
use lazy_static::lazy_static;

lazy_static! {
    /// Application state
    pub static ref APPLICATION: application::Lock<SearApplication> = application::Lock::default();
}

/// `sear` application
#[derive(Debug)]
pub struct SearApplication {
    /// Application configuration.
    config: Option<SearConfig>,

    /// Application state.
    state: application::State<Self>,
}

impl Default for SearApplication {
    fn default() -> Self {
        Self {
            config: None,
            state: application::State::default(),
        }
    }
}

impl Application for SearApplication {
    /// `sear` entrypoint command
    type Cmd = SearCommand;

    /// Configuration.
    type Cfg = SearConfig;

    /// Paths to resources within the application.
    type Paths = StandardPaths;

    /// Accessor for application configuration.
    fn config(&self) -> Option<&SearConfig> {
        self.config.as_ref()
    }

    /// Borrow the application state immutably.
    fn state(&self) -> &application::State<Self> {
        &self.state
    }

    /// Borrow the application state mutably.
    fn state_mut(&mut self) -> &mut application::State<Self> {
        &mut self.state
    }

    /// Register all components used by this application.
    fn register_components(&mut self, command: &Self::Cmd) -> Result<(), FrameworkError> {
        let components = self.framework_components(command)?;
        self.state.components.register(components)
    }

    /// Post-configuration lifecycle callback.
    fn after_config(&mut self, config: Option<Self::Cfg>) -> Result<(), FrameworkError> {
        // Provide configuration to all component `after_config()` handlers
        for component in self.state.components.iter_mut() {
            component.after_config(config.as_ref())?;
        }

        self.config = config;
        Ok(())
    }

    /// Get logging configuration from command-line options
    fn logging_config(&self, command: &SearCommand) -> LoggingConfig {
        if command.verbose {
            LoggingConfig::verbose()
        } else {
            LoggingConfig::default()
        }
    }
}
