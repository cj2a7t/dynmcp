use once_cell::sync::OnceCell;
use sqlx::MySqlPool;
use std::sync::Arc;

use crate::etcd::etcd_client_provider::EtcdClientProvider;

static ETCD_CLIENT: OnceCell<Arc<EtcdClientProvider>> = OnceCell::new();
static MYSQL_POOL: OnceCell<Arc<MySqlPool>> = OnceCell::new();

pub async fn init_etcd_global(
    etcd_endpoints: Vec<String>,
    etcd_username: String,
    etcd_password: String,
) -> anyhow::Result<()> {
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

pub async fn init_mysql_pool(database_url: &str) -> anyhow::Result<()> {
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
