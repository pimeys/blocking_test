use super::*;
use futures01::Future;
use futures03::future::TryFutureExt;
use std::sync::Arc;

pub struct Server<T>
where
    T: AsyncConnector + Send + Sync + 'static,
{
    db: Arc<T>,
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
            self.db.get_four().compat().map(|i| i.to_string())
        }
    }
}
