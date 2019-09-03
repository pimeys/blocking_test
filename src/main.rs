#![recursion_limit = "128"]

mod error;
mod postgresql;
mod server;

pub use error::*;
pub use postgresql::*;
pub use server::*;

use futures::future::FutureObj;
use http::status::StatusCode;
use server::Server;
use std::sync::Arc;
use tide::{error::ResultExt, response, App, Context, EndpointResult};

pub type Result<T> = std::result::Result<T, Error>;

pub trait IntoJson {
    fn into_json(self) -> crate::Result<serde_json::Value>;
}

pub trait AsyncConnector
where
    Self: Sync,
{
    fn run(&self, query: Arc<String>) -> FutureObj<'static, crate::Result<serde_json::Value>>;
}

async fn save_query(mut cx: Context<Server>) -> EndpointResult<()>
{
    let name = cx.param("name").client_err()?;
    let query = cx.body_string().await.client_err()?;

    cx.state().save(name, query).await;

    Ok(())
}

async fn run_query(cx: Context<Server>) -> EndpointResult
{
    let name: String = cx.param("name").client_err()?;

    match cx.state().run(&name).await {
        Ok(json) => Ok(response::json(json)),
        Err(crate::Error::NotFound) => Err(StatusCode::NOT_FOUND.into()),
        Err(e) => panic!(e),
    }
}

#[runtime::main(runtime_tokio::Tokio)]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let matches = clap::App::new("Database performance testing")
        .version("1.0")
        .arg(
            clap::Arg::with_name("threaded")
                .long("threaded")
                .help("Runs every query in a separate thread.")
                .takes_value(false)
                .required(false),
        )
        .arg(
            clap::Arg::with_name("sync")
                .long("sync")
                .help("Queries are run in the server thread, blocking all other requests.")
                .takes_value(false)
                .required(false),
        )
        .arg(
            clap::Arg::with_name("async")
                .long("async")
                .help("Queries are run asynchronously in an event loop.")
                .takes_value(false)
                .required(false),
        )
        .get_matches();

    let server = if matches.is_present("threaded") {
        println!("Multithread server listening on http://127.0.0.1:8080");

        let connector = postgresql::Threaded::new();
        Server::new(Box::new(connector))
    } else if matches.is_present("sync") {
        println!("Blocking server listening on http://127.0.0.1:8080");

        let connector = postgresql::Synchronous::new();
        Server::new(Box::new(connector))
    } else {
        println!("Asynchronous server listening on http://127.0.0.1:8080");

        let connector = postgresql::Asynchronous::new();
        Server::new(Box::new(connector))
    };

    let mut app = App::with_state(server);
    app.at("/:name").get(run_query).post(save_query);
    app.serve("127.0.0.1:8080").await?;

    Ok(())
}
