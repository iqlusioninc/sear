//! sear: CLI application (built on the Abscissa framework)

use crate::{commands::SearCommand, config::SearConfig};
use abscissa::application;
use abscissa::{Application, EntryPoint, FrameworkError, LoggingConfig, StandardPaths};
use lazy_static::lazy_static;

lazy_static! {
    /// Application state
    pub static ref APPLICATION: application::Lock<SearApplication> = application::Lock::default();
}

// Obtain a read-only (multi-reader) lock on the application state.
//
// Panics if the application state has not been initialized.
// pub fn app_reader() -> application::lock::Reader<SearApplication> {
//    APPLICATION.read()
// }

// Obtain an exclusive mutable lock on the application state.
// pub fn app_writer() -> application::lock::Writer<SearApplication> {
//    APPLICATION.write()
// }

// Obtain a read-only (multi-reader) lock on the application configuration.
//
// Panics if the application configuration has not been loaded.
// pub fn app_config() -> config::Reader<SearApplication> {
//    config::Reader::new(&APPLICATION)
// }

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
    type Cmd = EntryPoint<SearCommand>;

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
    fn logging_config(&self, command: &EntryPoint<SearCommand>) -> LoggingConfig {
        if command.verbose {
            LoggingConfig::verbose()
        } else {
            LoggingConfig::default()
        }
    }
}
