use std::sync::Arc;

use crate::datasource::datasource::DataSource;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use mcp_common::{
    cache::mcp_cache::McpCache,
    constants::constants::mcp_cache_consts::{ETCD_IDS_PREFIX, ETCD_TDS_PREFIX},
    etcd::etcd_client_provider::{EtcdEventType, EtcdWatchEvent},
    provider::global_provider::get_etcd,
    xds::{ids::IDS, tds::TDS},
};

pub struct EtcdDataSource {
    mcp_cache: Arc<McpCache>,
}

impl EtcdDataSource {
    pub fn new(mcp_cache: Arc<McpCache>) -> Self {
        Self { mcp_cache }
    }
}

#[async_trait]
impl DataSource for EtcdDataSource {
    async fn fetch_and_watch(self: Arc<Self>) -> Result<()> {
        let etcd = get_etcd();

        let tds_pairs = etcd.get_prefix(ETCD_TDS_PREFIX).await?;
        for (k, v) in tds_pairs {
            let tds: TDS = serde_json::from_str(&v)?;
            self.mcp_cache.insert_tds(k, tds);
        }
        let tds_cache = self.mcp_cache.clone();
        etcd.watch(ETCD_TDS_PREFIX, move |event: EtcdWatchEvent| {
            match event.event_type {
                EtcdEventType::Put => {
                    if let Some(val_str) = &event.value {
                        if let Ok(tds) = serde_json::from_str::<TDS>(val_str) {
                            tds_cache.insert_tds(event.key, tds);
                        } else {
                            eprintln!("Failed to parse TDS");
                        }
                    }
                }
                EtcdEventType::Delete => {
                    tds_cache.remove_tds(&event.key);
                }
                _ => {}
            }
        })
        .await?;

        let ids_pairs = etcd.get_prefix(ETCD_IDS_PREFIX).await?;
        for (k, v) in ids_pairs {
            let ids: IDS = serde_json::from_str(&v)?;
            self.mcp_cache.insert_ids(k, ids);
        }
        let ids_cache = self.mcp_cache.clone();
        etcd.watch(ETCD_IDS_PREFIX, move |event: EtcdWatchEvent| {
            match event.event_type {
                EtcdEventType::Put => {
                    if let Some(val_str) = &event.value {
                        if let Ok(ids) = serde_json::from_str::<IDS>(val_str) {
                            ids_cache.insert_ids(event.key, ids);
                        } else {
                            eprintln!("Failed to parse IDS");
                        }
                    }
                }
                EtcdEventType::Delete => {
                    ids_cache.remove_ids(&event.key);
                }
                _ => {}
            }
        })
        .await?;

        Ok(())
    }

    async fn put<T>(self: Arc<Self>, id: &str, value: &T) -> Result<T>
    where
        T: serde::Serialize + Clone + Send + Sync + 'static,
    {
        let etcd = get_etcd();
        let value_str = serde_json::to_string(value)?;
        etcd.put(id, &value_str).await?;
        Ok(value.clone())
    }

    async fn get<T>(self: Arc<Self>, id: &str) -> Result<T>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        let etcd = get_etcd();
        let value_opt = etcd.get(id).await?;
        match value_opt {
            Some(value_str) => Ok(serde_json::from_str(&value_str)?),
            None => Err(anyhow!("Key `{}` not found", id)),
        }
    }

    async fn delete(self: Arc<Self>, id: &str) -> Result<bool> {
        let etcd = get_etcd();
        etcd.delete(id).await
    }
}
