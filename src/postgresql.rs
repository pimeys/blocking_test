use super::*;
use futures01::{
    future::{err, lazy, ok, poll_fn},
    Future,
};
use futures03::compat::Future01CompatExt;
use postgres::{NoTls, Transaction as PostgresTransaction};
use r2d2_postgres::PostgresConnectionManager;
use std::sync::Arc;
use tokio_threadpool::blocking;

pub struct Postgres {
    pool: Arc<r2d2::Pool<PostgresConnectionManager<NoTls>>>,
}

impl AsyncConnector for Postgres {
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

    fn async_tx<F, T>(&self, f: F) -> DBIO<T>
    where
        T: Send + Sync + 'static,
        F: Fn(&mut dyn Transaction) -> Res<T> + Send + Sync + 'static,
    {
        let pool = self.pool.clone();

        let fut = lazy(move || {
            poll_fn(move || {
                blocking(|| {
                    let mut client = pool.get()?;
                    let mut tx = client.transaction()?;

                    let result = f(&mut tx);

                    if result.is_ok() {
                        tx.commit()?;
                    }

                    result
                })
            })
        })
        .then(|res| match res {
            Ok(Ok(val)) => ok(val),
            Ok(Err(val)) => err(val),
            Err(val) => err(val.into()),
        })
        .compat();

        DBIO(Box::pin(fut))
    }
}

impl<'a> Transaction for PostgresTransaction<'a> {
    fn filter(&mut self, q: &str) -> Res<Vec<i64>> {
        let stmt = self.prepare(q)?;
        let rows = self.query(&stmt, &[])?;
        let mut result = Vec::new();

        for row in rows {
            let i: i32 = row.get(0);
            result.push(i as i64)
        }

        Ok(result)
    }
}
