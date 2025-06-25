use std::sync::Arc;

use derive_new::new;
use mcp_core::cache::mcp_cache::McpCache;


#[derive(Clone, new)]
pub struct AppState {
    pub mcp_cache: Arc<McpCache>,
}
