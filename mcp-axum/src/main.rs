use std::sync::Arc;

use anyhow::{Ok, Result};
use clap::Parser;
use mcp_common::{
    cache::mcp_cache::McpCache,
    provider::global_provider::{init_etcd_global},
};
use mcp_plugin::datasource::{datasource::DataSource, ds_enum::DataSourceEnum, etcd_ds::EtcdDataSource};
use tokio::net::TcpListener;

use crate::{model::app_state::AppState, router::router::create_router};

mod error;
mod handler;
mod model;
mod router;
mod utils;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(long, default_value = "0.0.0.0:9999")]
    addr: String,
    #[arg(long, value_delimiter = ',', default_value = "http://localhost:2379")]
    etcd_endpoints: Vec<String>,
    #[arg(long, default_value = "")]
    etcd_username: String,
    #[arg(long, default_value = "")]
    etcd_password: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // args
    let args: Args = Args::parse();

    // global McpCache
    let mcp_cache: Arc<McpCache> = Arc::new(McpCache::new());

    // DataSource setup: Here we're using EtcdDataSource
    // But this could be replaced with other data sources (e.g., MySQL, Redis, etc.)
    init_etcd_global(args.etcd_endpoints, args.etcd_username, args.etcd_password).await?;
    let data_source = Arc::new(EtcdDataSource::new(mcp_cache.clone()));
    let data_source_enum = DataSourceEnum::Etcd(data_source.clone());
    DataSource::fetch_and_watch(data_source.clone()).await?;

    // init axum router
    let app_state: AppState = AppState::new(mcp_cache, Arc::new(data_source_enum));
    let router: axum::Router = create_router(app_state);

    // axum start
    let listener: TcpListener = TcpListener::bind(args.addr).await?;
    axum::serve(listener, router).await?;

    Ok(())
}
