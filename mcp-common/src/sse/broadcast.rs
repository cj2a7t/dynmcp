use anyhow::Result;
use std::sync::Arc;

use once_cell::sync::OnceCell;
use serde::Serialize;
use tokio::sync::broadcast::{self, Sender};

#[derive(Debug, Clone, Serialize)]
pub struct BroadcastMsg {
    pub ids_id: String,
    pub message: String,
}

static BROADCAST_TX: OnceCell<Arc<Sender<BroadcastMsg>>> = OnceCell::new();

pub fn get_broadcast_tx(capacity: Option<usize>) -> Result<Arc<Sender<BroadcastMsg>>> {
    BROADCAST_TX
        .get_or_try_init(|| {
            let (tx, _) = broadcast::channel::<BroadcastMsg>(capacity.unwrap_or(1024));
            Ok(Arc::new(tx))
        })
        .map(|arc| arc.clone())
}

pub fn get_global_broadcast_tx() -> Result<Arc<Sender<BroadcastMsg>>> {
    get_broadcast_tx(None)
}
