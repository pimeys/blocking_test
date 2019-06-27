use super::*;
use futures01::Future;
use futures03::future::TryFutureExt;

pub struct Server<T>
where
    T: AsyncConnector + Send + Sync + 'static,
{
    db: T,
}

impl_web! {
    impl<T> Server<T> where T: AsyncConnector + Send + Sync + 'static {
        pub fn new() -> Self {
            Self {
                db: T::new()
            }
        }

        #[get("/")]
        fn index(&self) -> impl Future<Item = String, Error = Error> + Send {
            self.db.get_four().compat().map(|i| i.to_string())
        }
    }
}
