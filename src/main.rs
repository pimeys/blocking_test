#![feature(async_await, await_macro)]
#![recursion_limit = "128"]

#[macro_use]
extern crate tower_web;

mod dbio;
mod error;
mod postgresql;
mod server;
mod sqlite;

pub use dbio::*;
pub use error::*;
pub use postgresql::*;
pub use server::*;
pub use sqlite::*;

use server::Server;
use tower_web::ServiceBuilder;

pub type Res<T> = Result<T, Error>;

pub trait Transaction {
    fn filter(&mut self, q: &str) -> Res<Vec<i64>>;
}

pub trait AsyncConnector {
    fn new() -> Self;

    fn async_tx<F, T>(&self, f: F) -> DBIO<T>
    where
        T: Send + Sync + 'static,
        F: Fn(&mut dyn Transaction) -> Res<T> + Send + Sync + 'static;
}

fn main() {
    let addr = "127.0.0.1:8080".parse().expect("Invalid address");
    println!("Listening on http://{}", addr);

    let server: Server<postgresql::Postgres> = Server::new();

    ServiceBuilder::new().resource(server).run(&addr).unwrap()
}
