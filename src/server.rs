use super::*;
use async_std::sync::RwLock;
use std::{collections::HashMap, sync::Arc};

pub struct Server
{
    db: Box<dyn AsyncConnector + Send + Sync + 'static>,
    queries: RwLock<HashMap<String, Arc<String>>>,
}

impl Server
{
    pub fn new(db: Box<dyn AsyncConnector + Send + Sync + 'static>) -> Self
    {
        Self {
            db,
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
            Some(query) => Ok(self.db.run(query.clone()).await?),
            None => Err(crate::Error::NotFound),
        }
    }
}
