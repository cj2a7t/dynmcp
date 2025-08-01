use dashmap::DashMap;
use std::sync::Arc;
use tracing::debug;

use crate::xds::{ids::IDS, tds::TDS};

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
        self.tds_map.get(id).map(|v| v.value().clone())
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
        debug!(?self.ids_map, "Full ids_map before lookup");
        self.ids_map.get(ids_id).map_or_else(
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
}
