use super::*;
use futures01::Future;
use futures03::future::{FutureExt, TryFutureExt};
use std::{sync::Arc};

pub struct Server<T>
where
    T: AsyncConnector + Send + Sync + 'static,
{
    db: Arc<T>,
}

/// Showing off with async/await just because...
async fn get_four<T>(db: Arc<T>) -> Res<i64>
where
    T: AsyncConnector + Send + Sync + 'static
{
    let two = db.async_tx(|tx| {
        let two = *tx.filter("SELECT 2").unwrap().first().unwrap();
        Ok(two)
    }).await?;

    Ok(two * 2)
}


impl_web! {
    impl<T> Server<T> where T: AsyncConnector + Send + Sync + 'static {
        pub fn new() -> Self {
            Self {
                db: Arc::new(T::new())
            }
        }

        #[get("/")]
        fn index(&self) -> impl Future<Item = String, Error = Error> + Send {
            get_four(self.db.clone()).boxed().compat().map(|i| i.to_string())
        }
    }
}
