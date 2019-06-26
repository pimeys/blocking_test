use super::*;
use futures::Future;
use std::sync::Arc;

pub struct Server<T>
where
    T: AsyncConnector + 'static,
{
    db: Arc<T>,
}

impl_web! {
    impl<T> Server<T> where T: AsyncConnector + 'static {
        pub fn new() -> Self {
            Self {
                db: Arc::new(T::new())
            }
        }

        #[get("/")]
        fn index(&self) -> impl Future<Item = String, Error = Error> + Send {
            self.db.async_tx(|tx| {
                let one = *tx.filter("SELECT 2").unwrap().first().unwrap();
                Ok(one.to_string())
            })
        }
    }
}
