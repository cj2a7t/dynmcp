use anyhow::{Error, Result};
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait DataSource {
    async fn fetch_and_watch(self: Arc<Self>) -> Result<()>;
    async fn put<T: serde::Serialize + Clone + Send + Sync + 'static>(
        id: &str,
        value: &T,
    ) -> Result<T, Error>;
    async fn get<T: for<'de> serde::Deserialize<'de>>(id: &str) -> Result<T, Error>;
    async fn delete(id: &str) -> Result<bool, Error>;
}
