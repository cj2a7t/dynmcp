use std::sync::Arc;
use dashmap::DashMap;
use mcp_common::etcd::etcd_client_provider::{ EtcdClientProvider, EtcdEventType, EtcdWatchEvent };
use anyhow::Result;

use crate::model::mcp_tool::McpTool;

const ETCD_TOOL_KEY_PREFIX: &str = "/mcp/apisix/tools/";

#[derive(Clone)]
pub struct McpCache {
    /// Core Resource Object: tools
    mcp_tool_map: Arc<DashMap<String, McpTool>>,
    mcp_tool_name_map: Arc<DashMap<String, String>>,
}

impl McpCache {
    pub fn new() -> Self {
        Self {
            mcp_tool_map: Arc::new(DashMap::new()),
            mcp_tool_name_map: Arc::new(DashMap::new())
        }
    }

    pub fn get_tool(&self, id: &str) -> Option<McpTool> {
        let key = format!("{}{}", ETCD_TOOL_KEY_PREFIX, id);
        self.mcp_tool_map.get(&key).map(|v| v.value().clone())
    }

    pub fn get_tool_by_name(&self, name: &str) -> Option<McpTool> {
        self.mcp_tool_name_map
            .get(name)
            .and_then(|tool_id_ref| {
                self.mcp_tool_map.get(tool_id_ref.key()).map(|v| v.value().clone())
            })
    }

    pub fn list_tools(&self) -> Vec<McpTool> {
        self.mcp_tool_map
            .iter()
            .map(|entry| entry.value().clone())
            .collect()
    }

    fn insert_tool(&self, key: String, value: McpTool) {
        self.mcp_tool_map.insert(key, value.clone());
        self.mcp_tool_name_map.insert(value.name, value.tool_ext_info.tool_id);
    }

    fn remove_tool(&self, key: &str) {
        if let Some(tool) = self.mcp_tool_map.get(key) {
            let name = tool.name.clone();
            self.mcp_tool_map.remove(key);
            self.mcp_tool_name_map.remove(&name);
        }
    }

    pub async fn start_watch(
        self: Arc<Self>,
        etcd: Arc<EtcdClientProvider>
    ) -> Result<()> {
        // tools
        let etcd_results = etcd.get_prefix(ETCD_TOOL_KEY_PREFIX).await?;
        for (k, v) in etcd_results {
            let tool: McpTool = serde_json::from_str(&v)?;
            self.insert_tool(k, tool);
        }
        etcd.watch(ETCD_TOOL_KEY_PREFIX, move |event: EtcdWatchEvent| {
            match event.event_type {
                EtcdEventType::Put => {
                    if let Some(val_str) = &event.value {
                        match serde_json::from_str::<McpTool>(val_str) {
                            Ok(tool) => {
                                self.insert_tool(event.key, tool);
                            }
                            Err(err) => {
                                eprintln!("Failed to parse tool JSON: {} => {}", event.key, err);
                            }
                        }
                    }
                }
                EtcdEventType::Delete => {
                    self.remove_tool(&event.key);
                }
                _ => {}
            }
        }).await?;
        Ok(())
    }
}
