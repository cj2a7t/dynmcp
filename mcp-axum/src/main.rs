use std::sync::Arc;

use anyhow::{Ok, Result, anyhow};
use mcp_common::{
    cache::mcp_cache::McpCache,
    log::log::init_logging,
    provider::global_provider::{get_app_config},
};
use mcp_plugin::datasource::factory::DataSourceFactory;
use tokio::net::TcpListener;
use tracing::info;

use crate::{model::app_state::AppState, router::router::create_router};

mod error;
mod handler;
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
    let app_state: AppState = AppState::new(mcp_cache, ds);
    let router: axum::Router = create_router(app_state);

    // axum start
    let addr = format!("{}:{}", config.app.host, config.app.port);
    let listener: TcpListener = TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;

    Ok(())
}
