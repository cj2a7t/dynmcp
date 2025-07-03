use std::sync::Arc;

use anyhow::{anyhow, Ok, Result};
use mcp_common::{
    cache::mcp_cache::McpCache, log::log::init_logging, provider::global_provider::get_app_config,
};
use mcp_plugin::datasource::factory::DataSourceFactory;
use tokio::net::TcpListener;
use tracing::info;

use crate::{model::app_state::AppState, router::router::create_router};

mod error;
mod handler;
mod extractor;
mod model;
mod router;

#[tokio::main]
async fn main() -> Result<()> {
    // init app config
    let config = get_app_config()?;

    // init logging
    let _guard = init_logging();

    // global McpCache
    let mcp_cache: Arc<McpCache> = Arc::new(McpCache::new());
    info!("McpCache initialized");

    // dataSource setup
    let ds = DataSourceFactory::build(mcp_cache.clone())
        .await
        .map_err(|e| anyhow!("Failed to create data source: {}", e))?;
    info!("DataSource initialized: {:?}", config.data_source);

    // init axum router
    let app_state: AppState = AppState::new(mcp_cache, ds, config.clone());
    let router: axum::Router = create_router(app_state);

    // axum start
    let addr = format!("{}:{}", config.app.host, config.app.port);
    info!("🚀 Starting Dynmcp HTTP server at http://{}", addr);
    let listener: TcpListener = TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;

    Ok(())
}
