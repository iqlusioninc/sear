//! `sear`: CLI application (built on the Abscissa framework)

use crate::{command::SearCmd, config::SearConfig};
use abscissa_core::{
    application::{self, AppCell},
    trace, Application, FrameworkError, StandardPaths,
};

/// Application state
pub static APPLICATION: AppCell<SearApp> = AppCell::new();

/// Obtain a read-only (multi-reader) lock on the application state.
///
/// Panics if the application state has not been initialized.
#[allow(dead_code)]
pub fn app_reader() -> application::lock::Reader<SearApp> {
    APPLICATION.read()
}

/// Obtain an exclusive mutable lock on the application state.
#[allow(dead_code)]
pub fn app_writer() -> application::lock::Writer<SearApp> {
    APPLICATION.write()
}

/// Obtain a read-only (multi-reader) lock on the application configuration.
///
/// Panics if the application configuration has not been loaded.
#[allow(dead_code)]
pub fn app_config() -> abscissa_core::config::Reader<SearApp> {
    abscissa_core::config::Reader::new(&APPLICATION)
}

/// `sear` application
#[derive(Debug)]
pub struct SearApp {
    /// Application configuration.
    config: Option<SearConfig>,

    /// Application state.
    state: application::State<Self>,
}

impl Default for SearApp {
    fn default() -> Self {
        Self {
            config: None,
            state: application::State::default(),
        }
    }
}

impl Application for SearApp {
    /// `sear` entrypoint command
    type Cmd = SearCmd;

    /// Configuration.
    type Cfg = SearConfig;

    /// Paths to resources within the application.
    type Paths = StandardPaths;

    /// Accessor for application configuration.
    fn config(&self) -> &SearConfig {
        self.config.as_ref().expect("config not loaded")
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
    fn after_config(&mut self, config: Self::Cfg) -> Result<(), FrameworkError> {
        self.state.components.after_config(&config)?;
        self.config = Some(config);
        Ok(())
    }

    /// Get tracing configuration from command-line options
    fn tracing_config(&self, command: &SearCmd) -> trace::Config {
        if command.verbose {
            trace::Config::verbose()
        } else {
            trace::Config::default()
        }
    }
}
