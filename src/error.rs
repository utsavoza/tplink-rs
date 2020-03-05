use std::error::Error as StdError;
use std::fmt;
use std::io;
use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
}

impl Error {
    pub(crate) fn new(kind: ErrorKind) -> Error {
        Error { kind }
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    Io(io::Error),
    Json(serde_json::Error),
    OperationNotSupported,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ErrorKind::Io(ref e) => e.fmt(f),
            ErrorKind::Json(ref e) => e.fmt(f),
            ErrorKind::OperationNotSupported => write!(f, "operation not supported"),
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
