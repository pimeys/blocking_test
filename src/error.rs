//! Herp derp -level error handling here.

use std::{error, fmt};
use tokio_threadpool::BlockingError;

#[derive(Debug)]
pub enum Error {
    R2d2,
    Rusqlite,
    Postgres,
    Other,
    Blocking,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Other => write!(f, "other"),
            Error::Rusqlite => write!(f, "rusqlite"),
            Error::R2d2 => write!(f, "r2d2"),
            Error::Blocking => write!(f, "blocking"),
            Error::Postgres => write!(f, "postgres"),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self {
            Error::Other => "other",
            Error::R2d2 => "r2d2",
            Error::Rusqlite => "rusqlite",
            Error::Blocking => "blocking",
            Error::Postgres => "postgres",
        }
    }

    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl From<r2d2::Error> for Error {
    fn from(_: r2d2::Error) -> Self {
        Error::R2d2
    }
}

impl From<rusqlite::Error> for Error {
    fn from(_: rusqlite::Error) -> Self {
        Error::Rusqlite
    }
}

impl From<tokio_postgres::error::Error> for Error {
    fn from(_: tokio_postgres::error::Error) -> Self {
        Error::Postgres
    }
}

impl From<BlockingError> for Error {
    fn from(_: BlockingError) -> Self {
        Error::Blocking
    }
}
