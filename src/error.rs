use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::io;

#[derive(Debug)]
pub enum Error {
    ParseError(wavefront_obj::ParseError),
    Io(io::Error),
    Error(String),
}

impl Display for Error {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref error) => error.fmt(formatter),
            Error::ParseError(ref error) => error.message.fmt(formatter),
            Error::Error(ref error) => error.fmt(formatter),
        }
    }
}

impl From<wavefront_obj::ParseError> for Error {
    fn from(error: wavefront_obj::ParseError) -> Self {
        Error::ParseError(error)
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::Io(error)
    }
}

impl From<String> for Error {
    fn from(error: String) -> Self {
        Error::Error(error)
    }
}

impl From<&str> for Error {
    fn from(error: &str) -> Self {
        Error::Error(String::from(error))
    }
}
