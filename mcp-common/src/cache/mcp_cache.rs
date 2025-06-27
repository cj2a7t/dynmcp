use anyhow::Result;
use dashmap::DashMap;
use std::sync::Arc;

use crate::{
    constants::constants::mcp_cache_consts::{ETCD_IDS_PREFIX, ETCD_TDS_PREFIX},
    etcd::etcd_client_provider::{EtcdClientProvider, EtcdEventType, EtcdWatchEvent},
    xds::{ids::IDS, tds::TDS},
};

#[derive(Clone)]
pub struct McpCache {
    // xDS Object: Tool Discovery Service (TDS)
    tds_map: Arc<DashMap<String, TDS>>,
    tds_name_map: Arc<DashMap<String, String>>,

    // xDS Object: Instance Discovery Service (IDS)
    ids_map: Arc<DashMap<String, IDS>>,
}

impl McpCache {
    pub fn new() -> Self {
        Self {
            tds_map: Arc::new(DashMap::new()),
            tds_name_map: Arc::new(DashMap::new()),
            ids_map: Arc::new(DashMap::new()),
        }
    }

    pub fn get_tds(&self, id: &str) -> Option<TDS> {
        let key = format!("{}{}", ETCD_TDS_PREFIX, id);
        self.tds_map.get(&key).map(|v| v.value().clone())
    }

    pub fn get_tds_by_name(&self, name: &str) -> Option<TDS> {
        self.tds_name_map.get(name).and_then(|tool_id_ref| {
            self.tds_map
                .get(tool_id_ref.key())
                .map(|v| v.value().clone())
        })
    }

    pub fn list_tds(&self) -> Vec<TDS> {
        self.tds_map
            .iter()
            .map(|entry| entry.value().clone())
            .collect()
    }

    pub fn insert_tds(&self, key: String, value: TDS) {
        self.tds_map.insert(key, value.clone());
        self.tds_name_map.insert(value.name, value.id);
    }

    pub fn remove_tds(&self, key: &str) {
        if let Some(tool) = self.tds_map.get(key) {
            let name = tool.name.clone();
            self.tds_map.remove(key);
            self.tds_name_map.remove(&name);
        }
    }

    pub fn list_tds_by_ids_id(&self, ids_id: &str) -> Vec<TDS> {
        let key = format!("{}{}", ETCD_IDS_PREFIX, ids_id);
        self.ids_map.get(&key).map_or_else(
            || vec![],
            |ids| {
                ids.tool_ids
                    .iter()
                    .filter_map(|tds_id| self.get_tds(tds_id))
                    .collect()
            },
        )
    }

    pub fn insert_ids(&self, key: String, value: IDS) {
        self.ids_map.insert(key, value.clone());
    }

    pub fn remove_ids(&self, key: &str) {
        self.ids_map.remove(key);
    }

    pub async fn start_watch(self: Arc<Self>, etcd: Arc<EtcdClientProvider>) -> Result<()> {
        // xDS Object: Tool Discovery Service (TDS)
        let self_tds = Arc::clone(&self);
        let tds_values = etcd.get_prefix(ETCD_TDS_PREFIX).await?;
        for (k, v) in tds_values {
            let tds: TDS = serde_json::from_str(&v)?;
            self.insert_tds(k, tds);
        }
        etcd.watch(ETCD_TDS_PREFIX, move |event: EtcdWatchEvent| {
            match event.event_type {
                EtcdEventType::Put => {
                    if let Some(val_str) = &event.value {
                        match serde_json::from_str::<TDS>(val_str) {
                            Ok(tds) => {
                                self_tds.insert_tds(event.key, tds);
                            }
                            Err(err) => {
                                // TODO
                                eprintln!("Failed to parse tool JSON: {} => {}", event.key, err);
                            }
                        }
                    }
                }
                EtcdEventType::Delete => {
                    self_tds.remove_tds(&event.key);
                }
                _ => {}
            }
        })
        .await?;

        // xDS Object: Instance Discovery Service (IDS)
        let self_ids = Arc::clone(&self);
        let etcd_results = etcd.get_prefix(ETCD_IDS_PREFIX).await?;
        for (k, v) in etcd_results {
            let ids: IDS = serde_json::from_str(&v)?;
            self.insert_ids(k, ids);
        }
        etcd.watch(ETCD_IDS_PREFIX, move |event: EtcdWatchEvent| {
            match event.event_type {
                EtcdEventType::Put => {
                    if let Some(val_str) = &event.value {
                        match serde_json::from_str::<IDS>(val_str) {
                            Ok(ids) => {
                                self_ids.insert_ids(event.key, ids);
                            }
                            Err(err) => {
                                // TODO
                                eprintln!("Failed to parse tool JSON: {} => {}", event.key, err);
                            }
                        }
                    }
                }
                EtcdEventType::Delete => {
                    self_ids.remove_ids(&event.key);
                }
                _ => {}
            }
        })
        .await?;

        Ok(())
    }
}
