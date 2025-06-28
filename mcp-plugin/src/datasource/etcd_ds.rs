use std::sync::Arc;

use crate::datasource::datasource::DataSource;
use anyhow::{Error, Result};
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
        let datasource = Arc::clone(&self);
        let etcd = get_etcd();

        // xDS: Tool Discovery Service (TDS)
        let tds_values = etcd.get_prefix(ETCD_TDS_PREFIX).await?;
        let tds_mcp_cache_ref = datasource.mcp_cache.clone();
        for (k, v) in tds_values {
            let tds: TDS = serde_json::from_str(&v)?;
            datasource.mcp_cache.insert_tds(k, tds);
        }

        etcd.watch(ETCD_TDS_PREFIX, move |event: EtcdWatchEvent| {
            match event.event_type {
                EtcdEventType::Put => {
                    if let Some(val_str) = &event.value {
                        match serde_json::from_str::<TDS>(val_str) {
                            Ok(tds) => {
                                tds_mcp_cache_ref.insert_tds(event.key, tds);
                            }
                            Err(_err) => {
                                eprintln!("TODO");
                            }
                        }
                    }
                }
                EtcdEventType::Delete => {
                    tds_mcp_cache_ref.remove_tds(&event.key);
                }
                _ => {}
            }
        })
        .await?;

        //xDS:Instance Discovery Service (IDS)
        let etcd_results = etcd.get_prefix(ETCD_IDS_PREFIX).await?;
        let ids_mcp_cache_ref = datasource.mcp_cache.clone();

        for (k, v) in etcd_results {
            let ids: IDS = serde_json::from_str(&v)?;
            ids_mcp_cache_ref.insert_ids(k, ids);
        }

        etcd.watch(ETCD_IDS_PREFIX, move |event: EtcdWatchEvent| {
            match event.event_type {
                EtcdEventType::Put => {
                    if let Some(val_str) = &event.value {
                        match serde_json::from_str::<IDS>(val_str) {
                            Ok(ids) => {
                                ids_mcp_cache_ref.insert_ids(event.key, ids);
                            }
                            Err(_err) => {
                                eprintln!("TODO");
                            }
                        }
                    }
                }
                EtcdEventType::Delete => {
                    ids_mcp_cache_ref.remove_ids(&event.key);
                }
                _ => {}
            }
        })
        .await?;

        Ok(())
    }

    async fn put<T: serde::Serialize + Clone + Send + Sync + 'static>(
        self: Arc<Self>,
        id: &str,
        value: &T,
    ) -> Result<T, Error> {
        let etcd = get_etcd();
        let value_str = serde_json::to_string(value)?;
        etcd.put(id, &value_str).await?;
        Ok(value.clone())
    }

    async fn get<T: for<'de> serde::Deserialize<'de>>(
        self: Arc<Self>,
        id: &str,
    ) -> Result<T, Error> {
        let etcd = get_etcd();
        let value_opt = etcd.get(id).await?;
        match value_opt {
            Some(value_str) => {
                let value = serde_json::from_str::<T>(&value_str)?;
                Ok(value)
            }
            None => Err(anyhow::anyhow!("Key not found")),
        }
    }

    async fn delete(self: Arc<Self>, id: &str) -> Result<bool, Error> {
        let etcd = get_etcd();
        let deleted = etcd.delete(id).await?;
        Ok(deleted)
    }
}
