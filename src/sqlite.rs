use super::*;
use futures01::{
    future::{err, lazy, ok, poll_fn},
    Future,
};
use futures03::compat::Future01CompatExt;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{Transaction as SqliteTransaction, NO_PARAMS};
use std::sync::Arc;
use tokio_threadpool::blocking;

pub struct Sqlite {
    pool: Arc<r2d2::Pool<SqliteConnectionManager>>,
}

impl AsyncConnector for Sqlite {
    fn new() -> Self {
        let pool = r2d2::Pool::builder()
            .build(SqliteConnectionManager::memory())
            .unwrap();
        let pool = Arc::new(pool);

        Sqlite { pool }
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
                    let mut conn = pool.get()?;
                    let mut tx = conn.transaction()?;
                    tx.set_prepared_statement_cache_capacity(65536);

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

impl<'a> Transaction for SqliteTransaction<'a> {
    fn filter(&mut self, q: &str) -> Res<Vec<i64>> {
        let mut stmt = self.prepare_cached(q)?;
        let mut rows = stmt.query(NO_PARAMS)?;
        let mut result = Vec::new();

        while let Some(row) = rows.next()? {
            result.push(row.get(0).unwrap());
        }

        Ok(result)
    }
}
