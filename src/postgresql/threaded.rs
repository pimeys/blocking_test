use crate::{AsyncConnector, IntoJson};
use futures::future::FutureObj;
use postgres::NoTls;
use r2d2_postgres::PostgresConnectionManager;
use std::sync::Arc;
use tokio_executor::blocking;

pub struct Threaded {
    pool: Arc<r2d2::Pool<PostgresConnectionManager<NoTls>>>,
}

impl AsyncConnector for Threaded {
    fn new() -> Self {
        let manager = PostgresConnectionManager::new(
            "user = postgres host = localhost password = prisma"
                .parse()
                .unwrap(),
            NoTls,
        );

        let pool = Arc::new(r2d2::Pool::builder().build(manager).unwrap());

        Self { pool }
    }

    fn run(&self, query: Arc<String>) -> FutureObj<'static, crate::Result<serde_json::Value>> {
        let pool = self.pool.clone();

        let fut = blocking::run(move || {
            let mut client = pool.get()?;
            let rows = client.query(query.as_str(), &[])?;

            Ok(rows.into_json()?)
        });

        FutureObj::new(Box::new(fut))
    }
}
