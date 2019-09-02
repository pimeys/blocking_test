use std::{error, fmt};

#[derive(Debug)]
pub enum Error {
    R2d2,
    Postgres,
    Other,
    NotFound,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Other => write!(f, "other"),
            Error::R2d2 => write!(f, "r2d2"),
            Error::Postgres => write!(f, "postgres"),
            Error::NotFound => write!(f, "not_found"),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self {
            Error::Other => "other",
            Error::R2d2 => "r2d2",
            Error::Postgres => "postgres",
            Error::NotFound => "not_found",
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

impl From<tokio_postgres::error::Error> for Error {
    fn from(_: tokio_postgres::error::Error) -> Self {
        Error::Postgres
    }
}
