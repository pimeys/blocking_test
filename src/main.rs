#![recursion_limit = "128"]

#[macro_use]
extern crate tower_web;

pub mod postgresql;
pub mod server;
pub mod sqlite;

use futures::Future;
use postgresql::Postgres;
use server::Server;
use sqlite::Sqlite;
use std::{error, fmt};
use tokio_threadpool::BlockingError;
use tower_web::ServiceBuilder;

pub type Res<T> = Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    R2d2,
    Rusqlite,
    Postgres,
    Other,
    Blocking,
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

pub trait Transaction {
    fn filter(&mut self, q: &str) -> Res<Vec<i64>>;
}

pub type FutRes<T> = Box<dyn Future<Item = T, Error = Error> + Send>;

pub trait AsyncConnector {
    fn new() -> Self;

    fn async_tx<F, T>(&self, f: F) -> FutRes<T>
    where
        T: Send + Sync + 'static,
        F: Fn(&mut dyn Transaction) -> Res<T> + Send + Sync + 'static;
}

fn main() {
    let addr = "127.0.0.1:8080".parse().expect("Invalid address");
    println!("Listening on http://{}", addr);

    let server: Server<Postgres> = Server::new();

    ServiceBuilder::new().resource(server).run(&addr).unwrap()
}
