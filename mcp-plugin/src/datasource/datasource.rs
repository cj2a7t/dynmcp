use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait DataSource {
    async fn fetch_and_watch(self: Arc<Self>) -> Result<()>;

    async fn put<T>(self: Arc<Self>, id: &str, value: &T) -> Result<T>
    where
        T: serde::Serialize + Clone + Send + Sync + 'static;

    async fn get<T>(self: Arc<Self>, id: &str) -> Result<T>
    where
        T: for<'de> serde::Deserialize<'de>;

    async fn delete(self: Arc<Self>, id: &str) -> Result<bool>;
}
