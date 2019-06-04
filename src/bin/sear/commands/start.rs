//! `start` subcommand - example of how to write a subcommand

/// App-local prelude includes `app_reader()`/`app_writer()`/`app_config()`
/// accessors along with logging macros. Customize as you see fit.
#[allow(unused_imports)]
use crate::prelude::*;

use abscissa::{Command, Options, Runnable};

/// `start` subcommand
///
/// The `Options` proc macro generates an option parser based on the struct
/// definition, and is defined in the `gumdrop` crate. See their documentation
/// for a more comprehensive example:
///
/// <https://docs.rs/gumdrop/>
#[derive(Command, Debug, Options)]
pub struct StartCommand {
    /// To whom are we saying hello?
    #[options(free)]
    recipient: Option<String>,
}

impl Runnable for StartCommand {
    /// Print "Hello, world!"
    fn run(&self) {
        match &self.recipient {
            Some(recipient) => println!("Hello, {}!", recipient),
            None => Self::print_usage(&[]),
        }
    }
}
