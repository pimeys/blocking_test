use super::*;
use futures::future::{err, lazy, ok, poll_fn};
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{Connection, Transaction as SqliteTransaction, NO_PARAMS};
use std::sync::Arc;
use tokio_threadpool::blocking;

pub struct Sqlite {
    pool: Arc<r2d2::Pool<SqliteConnectionManager>>,
}

impl Sqlite {
    fn with_connection<F, T>(&self, f: F) -> Res<T>
    where
        F: FnOnce(&mut Connection) -> Res<T>,
    {
        let mut conn = self.pool.get()?;
        f(&mut conn)
    }
}

impl SyncConnector for Sqlite {
    fn with_transaction<F, T>(&self, f: F) -> Res<T>
    where
        F: FnOnce(&mut dyn Transaction) -> Res<T>,
    {
        self.with_connection(|ref mut conn| {
            let mut tx = conn.transaction()?;
            tx.set_prepared_statement_cache_capacity(65536);

            let result = f(&mut tx);

            if result.is_ok() {
                tx.commit()?;
            }

            result
        })
    }
}

impl AsyncConnector for Sqlite {
    fn new() -> Self {
        let pool = r2d2::Pool::builder()
            .build(SqliteConnectionManager::memory())
            .unwrap();
        let pool = Arc::new(pool);

        Sqlite { pool }
    }

    fn async_tx<F, T>(&self, f: F) -> FutRes<T>
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
        });

        Box::new(fut)
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
