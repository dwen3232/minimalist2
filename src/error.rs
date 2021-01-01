use std::error::Error as StdError;
use std::fmt;
use std::result;

use image::error;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    ModelError,
    WriteError,
}

impl StdError for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ModelError => write!(f, "Model Error"),
            Error::WriteError => write!(f, "Write Error"),
        }
    }
}

impl From<error::ImageError> for Error {
    fn from(_: error::ImageError) -> Self {
        Error::ModelError
    }
}