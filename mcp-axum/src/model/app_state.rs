use std::sync::Arc;

use dashmap::DashMap;
use derive_new::new;
use mcp_common::{cache::mcp_cache::McpCache, config::config::AppConfig};
use mcp_plugin::datasource::ds_enum::DataSourceEnum;
use tokio::sync::mpsc::UnboundedSender;

#[derive(Clone, new)]
pub struct AppState {
    pub mcp_cache: Arc<McpCache>,
    pub data_source: Arc<DataSourceEnum>,
    pub config: Arc<AppConfig>,
    pub session_manager: Arc<SessionManager>,
}

impl AsRef<AppState> for AppState {
    fn as_ref(&self) -> &AppState {
        self
    }
}

pub struct SessionManager {
    pub sessions: DashMap<String, UnboundedSender<String>>,
}
