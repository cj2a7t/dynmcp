use anyhow::{anyhow, Result};
use once_cell::sync::OnceCell;
use sqlx::MySqlPool;
use std::sync::Arc;

use crate::{
    etcd::etcd_client_provider::EtcdClientProvider,
    http_client::http_client_provider::HttpClientProvider,
};

static ETCD_CLIENT: OnceCell<Arc<EtcdClientProvider>> = OnceCell::new();
static MYSQL_POOL: OnceCell<Arc<MySqlPool>> = OnceCell::new();
static HTTP_CLIENT: OnceCell<Arc<HttpClientProvider>> = OnceCell::new();

pub async fn init_etcd_global(
    etcd_endpoints: Vec<String>,
    etcd_username: String,
    etcd_password: String,
) -> Result<()> {
    let client =
        Arc::new(EtcdClientProvider::new(etcd_endpoints, etcd_username, etcd_password).await?);
    ETCD_CLIENT
        .set(client)
        .map_err(|_| anyhow::anyhow!("Etcd already initialized"))?;
    Ok(())
}

pub fn get_etcd() -> Arc<EtcdClientProvider> {
    ETCD_CLIENT.get().expect("Etcd not initialized").clone()
}

pub async fn init_mysql_pool(database_url: &str) -> Result<()> {
    let pool = MySqlPool::connect(database_url).await?;
    MYSQL_POOL
        .set(Arc::new(pool))
        .map_err(|_| anyhow::anyhow!("MySQL Pool already initialized"))?;
    Ok(())
}

pub fn get_mysql_pool() -> Arc<MySqlPool> {
    MYSQL_POOL
        .get()
        .expect("MySQL pool not initialized")
        .clone()
}

pub fn init_http_client() -> Result<()> {
    let client = Arc::new(HttpClientProvider::new()?);
    HTTP_CLIENT
        .set(client)
        .map_err(|_| anyhow!("HTTP client already initialized"))?;
    Ok(())
}
pub fn get_http_client() -> Arc<HttpClientProvider> {
    HTTP_CLIENT
        .get()
        .expect("HTTP client not initialized")
        .clone()
}
