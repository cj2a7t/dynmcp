use once_cell::sync::OnceCell;
use std::sync::Arc;

use crate::etcd::etcd_client_provider::EtcdClientProvider;

static ETCD_CLIENT: OnceCell<Arc<EtcdClientProvider>> = OnceCell::new();

pub async fn init_etcd_global(
    etcd_endpoints: Vec<String>,
    etcd_username: String,
    etcd_password: String
) -> anyhow::Result<()> {
    let client = Arc::new(
        EtcdClientProvider::new(etcd_endpoints, etcd_username, etcd_password).await?
    );
    ETCD_CLIENT.set(client).map_err(|_| anyhow::anyhow!("Etcd already initialized"))?;
    Ok(())
}

pub fn get_etcd() -> Arc<EtcdClientProvider> {
    ETCD_CLIENT.get().expect("Etcd not initialized").clone()
}
