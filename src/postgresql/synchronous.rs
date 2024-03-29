use crate::{AsyncConnector, IntoJson};
use futures::future::{self, FutureObj};
use postgres::NoTls;
use r2d2_postgres::PostgresConnectionManager;
use std::{sync::Arc, time::Duration};

pub struct Synchronous {
    pool: Arc<r2d2::Pool<PostgresConnectionManager<NoTls>>>,
}

impl Synchronous {
    pub fn new() -> Self {
        let manager = PostgresConnectionManager::new(
            "user = postgres host = localhost password = prisma"
                .parse()
                .unwrap(),
            NoTls,
        );

        let builder = r2d2::Pool::builder()
            .max_size(10)
            .connection_timeout(Duration::from_secs(5))
            .test_on_check_out(false);

        let pool = Arc::new(builder.build(manager).unwrap());

        Self { pool }
    }
}

impl AsyncConnector for Synchronous {
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
