use crate::{AsyncConnector, IntoJson};
use futures::future::{FutureObj, self};
use postgres::NoTls;
use r2d2_postgres::PostgresConnectionManager;
use std::sync::Arc;

pub struct Synchronous {
    pool: Arc<r2d2::Pool<PostgresConnectionManager<NoTls>>>,
}

impl AsyncConnector for Synchronous {
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
        let fetch_json = || {
            let mut client = self.pool.get()?;
            let rows = client.query(query.as_str(), &[])?;

            Ok(rows.into_json()?)
        };

        match fetch_json() {
            Ok(json) => FutureObj::new(Box::new(future::ok(json))),
            Err(e) => FutureObj::new(Box::new(future::err(e))),
        }
    }
}
