use std::error::Error as StdError;
use std::fmt;
use std::io;
use std::result;

/// A type alias for `Result<T, tplink::Error>`.
pub type Result<T> = result::Result<T, Error>;

/// Errors that may occur while interacting with a device.
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
}

impl Error {
    /// Creates a new `Error` for a given type of this error.
    pub(crate) fn new(kind: ErrorKind) -> Error {
        Error { kind }
    }

    /// Returns the specific type of this error.
    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}

/// The specific type of an error.
#[derive(Debug)]
pub enum ErrorKind {
    /// An I/O error that occurred while interacting with a device.
    Io(io::Error),
    /// An error of this kind occurs when performing automatic
    /// serialization/deserialization with serde.
    Json(serde_json::Error),
    /// An error of this kind occurs when an operation requested by
    /// the client is not supported by the device.
    UnsupportedOperation(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ErrorKind::Io(ref e) => e.fmt(f),
            ErrorKind::Json(ref e) => e.fmt(f),
            ErrorKind::UnsupportedOperation(ref op) => write!(f, "unsupported operation: {}", op),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self.kind {
            ErrorKind::Io(ref e) => Some(e),
            ErrorKind::Json(ref e) => Some(e),
            _ => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::new(ErrorKind::Io(e))
    }
}

pub(crate) fn json(e: serde_json::Error) -> Error {
    Error::new(ErrorKind::Json(e))
}

pub(crate) fn unsupported_operation(name: &str) -> Error {
    Error::new(ErrorKind::UnsupportedOperation(name.into()))
}
