use std::error;
use std::error::Error;
use std::fmt;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]

pub enum ErrorKind {
    /// An error with the TCP connection
    TCPError,
}

/// Implemetation for Clone trait
impl Clone for ErrorKind {
    fn clone(&self) -> Self {
        use self::ErrorKind::*;
        match self {
            TCPError => TCPError,
        }
    }
}

#[derive(Debug, Error, Clone)]
pub struct Error {
    kind: ErrorKind,
    // here, trust has backtrace!
}

impl Error {
    /// Get the kind of the error
    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::ErrorKind::*;
        match self.kind {
            TCPError => write!(f, "TCP Error"),
        }
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error { kind }
    }
}