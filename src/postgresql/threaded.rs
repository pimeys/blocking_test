use crate::{AsyncConnector, IntoJson};
use futures::future::FutureObj;
use postgres::NoTls;
use r2d2_postgres::PostgresConnectionManager;
use std::sync::Arc;
use tokio_executor::blocking;

pub struct Threaded {
    pool: Arc<r2d2::Pool<PostgresConnectionManager<NoTls>>>,
}

impl Threaded {
    pub fn new() -> Self {
        let manager = PostgresConnectionManager::new(
            "user = postgres host = localhost password = prisma"
                .parse()
                .unwrap(),
            NoTls,
        );

        let builder = r2d2::Pool::builder().max_size(10).test_on_check_out(false);
        let pool = Arc::new(builder.build(manager).unwrap());

        Self { pool }
    }

}

impl AsyncConnector for Threaded {
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
