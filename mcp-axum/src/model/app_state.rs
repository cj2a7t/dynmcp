use std::sync::Arc;

use derive_new::new;
use mcp_common::cache::mcp_cache::McpCache;
use mcp_plugin::datasource::datasource::DataSource;

#[derive(Clone, new)]
pub struct AppState {
    pub mcp_cache: Arc<McpCache>,
    pub data_source: Arc<dyn DataSource + Send + Sync>,
}
