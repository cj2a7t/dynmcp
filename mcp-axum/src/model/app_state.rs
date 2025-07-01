use std::sync::Arc;

use derive_new::new;
use mcp_common::{cache::mcp_cache::McpCache, config::config::AppConfig};
use mcp_plugin::datasource::ds_enum::DataSourceEnum;

#[derive(Clone, new)]
pub struct AppState {
    pub mcp_cache: Arc<McpCache>,
    pub data_source: Arc<DataSourceEnum>,
    pub config: Arc<AppConfig>,
}

impl AsRef<AppState> for AppState {
    fn as_ref(&self) -> &AppState {
        self
    }
}
