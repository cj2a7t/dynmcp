use std::sync::Arc;

use anyhow::{Ok, Result};
use clap::Parser;
use mcp_common::provider::global_provider::{get_etcd, init_etcd_global};
use mcp_core::cache::mcp_cache::McpCache;
use tokio::net::TcpListener;

use crate::{model::app_state::AppState, router::router::create_router};

mod handler;
mod model;
mod router;
mod error;
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

    // init etcd
    init_etcd_global(args.etcd_endpoints, args.etcd_username, args.etcd_password).await?;

    // build and watch local cache from etcd
    let etcd = get_etcd();
    let mcp_cache: Arc<McpCache> = Arc::new(McpCache::new());
    let mcp_cache_for_watch = mcp_cache.clone();
    mcp_cache_for_watch.start_watch(etcd.clone()).await?;

    // init axum router
    let app_state: AppState = AppState::new(mcp_cache);
    let router: axum::Router = create_router(app_state);

    // axum start
    let listener: TcpListener = TcpListener::bind(args.addr).await?;
    axum::serve(listener, router).await?;

    Ok(())
}
