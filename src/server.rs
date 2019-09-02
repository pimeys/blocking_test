use super::*;
use std::{collections::HashMap, sync::Arc};
use async_std::sync::RwLock;

pub struct Server<T>
where
    T: AsyncConnector + Send + Sync + 'static,
{
    db: T,
    queries: RwLock<HashMap<String, Arc<String>>>,
}

impl<T> Server<T> where T: AsyncConnector + Send + Sync + 'static {
    pub fn new() -> Self {
        Self {
            db: T::new(),
            queries: RwLock::new(HashMap::new()),
        }
    }

    pub async fn save(&self, name: String, query: String) {
        let mut queries = self.queries.write().await;
        queries.insert(name, Arc::new(query));
    }

    pub async fn run(&self, query_name: &str) -> crate::Result<serde_json::Value> {
        let queries = self.queries.read().await;

        match queries.get(query_name) {
            Some(query) => {
                Ok(self.db.run(query.clone()).await?)
            },
            None => {
                Err(crate::Error::NotFound)
            }
        }
    }
}
