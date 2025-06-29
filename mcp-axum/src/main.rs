use std::sync::Arc;

use anyhow::{Ok, Result};
use clap::Parser;
use mcp_common::{cache::mcp_cache::McpCache, DynMcpArgs};
use mcp_plugin::datasource::factory::DataSourceFactory;
use tokio::net::TcpListener;

use crate::{model::app_state::AppState, router::router::create_router};

mod error;
mod handler;
mod model;
mod router;

#[tokio::main]
async fn main() -> Result<()> {
    // args
    let args: DynMcpArgs = DynMcpArgs::parse();

    // global McpCache
    let mcp_cache: Arc<McpCache> = Arc::new(McpCache::new());

    // DataSource setup: Here we're using MysqlDataSource as an example.
    // You can use EtcdDataSource or any other data source as needed.
    // The DataSourceFactory will create the appropriate data source based on the args.
    // For example, if args.data_source is "etcd", it will create an EtcdDataSource.
    let ds_factory = args.data_source.parse::<DataSourceFactory>()?;
    let args_clone = args.clone();
    let ds = ds_factory
        .create_data_source_enum(args, mcp_cache.clone())
        .await?;

    // init axum router
    let app_state: AppState = AppState::new(mcp_cache, ds);
    let router: axum::Router = create_router(app_state);

    // axum start
    let listener: TcpListener = TcpListener::bind(args_clone.addr).await?;
    axum::serve(listener, router).await?;

    Ok(())
}
