use crate::{AsyncConnector, IntoJson, Error};
use futures::{future::FutureObj, compat::Future01CompatExt};
use futures01::{Stream, future::{Future as _, Either, err}};
use postgres::NoTls;
use bb8_postgres::PostgresConnectionManager;
use std::sync::Arc;

pub struct Asynchronous {
    pool: bb8::Pool<PostgresConnectionManager<NoTls>>,
}

impl Asynchronous {
    pub fn new() -> Self {
        let manager = PostgresConnectionManager::new(
            "user = postgres host = localhost password = prisma",
            NoTls,
        );

        let builder = bb8::Pool::builder().max_size(10).test_on_check_out(false);
        let pool = builder.build_unchecked(manager);

        Self { pool }
    }
}

impl AsyncConnector for Asynchronous {
    fn run(&self, query: Arc<String>) -> FutureObj<'static, crate::Result<serde_json::Value>> {
        // Please god help me...
        //
        // this kind of bullshit will be history when the ecosystem finally
        // implements std::future.
        let fut = self.pool.run(move |mut client| {
            client.prepare(query.as_str()).then(move |res| match res {
                Ok(stmt) => {
                    let f = client
                        .query(&stmt, &[])
                        .collect()
                        .then(move |res| match res {
                            Ok(rows) => Ok((rows.into_json().unwrap(), client)),
                            Err(_) => Err((Error::Postgres, client)),
                        });

                    Either::A(f)
                },
                Err(_) => Either::B(err((Error::Postgres, client))),
            })
        }).map_err(|_| Error::Bb8).compat();

        FutureObj::new(Box::new(fut))
    }
}
