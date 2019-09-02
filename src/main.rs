#![recursion_limit = "128"]

mod error;
mod postgresql;
mod server;

pub use error::*;
pub use postgresql::*;
pub use server::*;

use server::Server;
use futures::future::FutureObj;
use tide::{response, App, Context, EndpointResult, error::ResultExt};
use http::status::StatusCode;

pub type Result<T> = std::result::Result<T, Error>;

pub trait Transaction {
    fn filter(&mut self, q: &str) -> crate::Result<Vec<i64>>;
}

pub trait AsyncConnector where Self: Sync {
    fn new() -> Self;
    fn run(&self, query: String) -> FutureObj<'static, crate::Result<serde_json::Value>>;
}

async fn save_query<T>(mut cx: Context<Server<T>>) -> EndpointResult<()>
where
    T: AsyncConnector + Send + Sync + 'static
{
    let name = cx.param("name").client_err()?;
    let query = cx.body_string().await.client_err()?;

    cx.state().save(name, query).await;

    Ok(())
}

async fn run_query<T>(cx: Context<Server<T>>) -> EndpointResult
where
    T: AsyncConnector + Send + Sync + 'static
{
    let name: String = cx.param("name").client_err()?;

    match cx.state().run(&name).await {
        Ok(json) => Ok(response::json(json)),
        Err(crate::Error::NotFound) => Err(StatusCode::NOT_FOUND.into()),
        Err(e) => panic!(e)
    }
}

fn main() {
    println!("Listening on http://127.0.0.1:8080");

    let server: Server<postgresql::Postgres> = Server::new();
    let mut app = App::with_state(server);

    app.at("/:name").get(run_query).post(save_query);
    app.run("127.0.0.1:8080").unwrap();
}
