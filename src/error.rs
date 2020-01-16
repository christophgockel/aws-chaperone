use std::convert::From;
use std::fmt;
use std::io;

use crate::config;

#[derive(Debug)]
pub enum ChaperoneError {
    MissingProfile,
    CommandNotFound(String),
    ConfigurationError(config::Error),
    IoError(io::Error),
}

impl From<io::Error> for ChaperoneError {
    fn from(error: io::Error) -> Self {
        ChaperoneError::IoError(error)
    }
}

impl fmt::Display for ChaperoneError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match *self {
            ChaperoneError::MissingProfile => f.write_str("No CHAPERONE_PROFILE defined."),
            ChaperoneError::CommandNotFound(ref name) => {
                f.write_str(&format!("Command not found: {}", name))
            }
            ChaperoneError::ConfigurationError(ref error) => {
                f.write_str(&format!("Error with configuration: {}", error))
            }
            ChaperoneError::IoError(ref error) => f.write_str(&format!("I/O Error: {}", error)),
        }
    }
}

impl From<config::Error> for ChaperoneError {
    fn from(error: config::Error) -> Self {
        ChaperoneError::ConfigurationError(error)
    }
}
