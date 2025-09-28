use std::{convert::Infallible, sync::Arc, time::Duration};

use axum::{
    extract::{Path, State},
    http::{HeaderMap, HeaderValue},
    response::{sse::Event, IntoResponse, Response, Sse},
    Json,
};
use futures::{stream::select, StreamExt};
use mcp_common::{
    enums::ids_protocol_type::IdsProtoType,
    sse::broadcast::get_global_broadcast_tx,
    utils::{header_builder::HeaderBuilder, header_extractor::HeaderExtractor},
    xds::ids::IDSMetadata,
};
use mcp_core::{
    error::dyn_execute_error::DynExecuteError,
    mcp::protocol::mcp_protocol::{self, Requestx},
};
use serde_json::{from_str, Value};
use tokio::time::interval;
use tokio_stream::wrappers::{errors::BroadcastStreamRecvError, BroadcastStream, IntervalStream};
use tracing::error;

use crate::{
    error::api_error::RestAPIError,
    model::{app_state::AppState, jsonrpc_response::JSONRpcResponse, sse_response::once_sse},
};

pub async fn mcp_post(
    headers: HeaderMap,
    Path(ids_id): Path<String>,
    State(state): State<AppState>,
    Json(jsonrpc_request): Json<Value>,
) -> Result<Response, RestAPIError> {
    // create a request context for the MCP protocol
    let reqx = Requestx {
        mcp_cache: &state.mcp_cache,
        ids_id: &ids_id,
    };

    // find ids
    let ids = state
        .mcp_cache
        .get_ids(&ids_id)
        .ok_or_else(|| RestAPIError::for_json_rpc(DynExecuteError::IdsNotFound))?;

    // proto type
    let ids_metadata: IDSMetadata = from_str(ids.metadata.as_str())?;

    // execute dynamic mcp protocol
    let result = mcp_protocol::execute_dyn(jsonrpc_request, &reqx)
        .await
        .map_err(|err| RestAPIError::for_json_rpc(err))?;

    // build response by ids protocol type
    let proto_type: IdsProtoType = ids_metadata.proto_type.as_str().into();
    let mut response = match proto_type {
        IdsProtoType::StreamableStateless => {
            JSONRpcResponse::with_u16_status(result.respx.http_status, result.response)
                .into_response()
        }
        IdsProtoType::Other(_) => once_sse(&result.response),
    };

    // Use HeaderExtractor to get session ID
    let header_extractor = HeaderExtractor::new(&headers);
    let session_id = header_extractor.get_str("Mcp-Session-Id");
    let session_value = result
        .respx
        .initialize_session_id
        .or_else(|| session_id)
        .unwrap_or_default();

    // Use HeaderBuilder to set response headers
    HeaderBuilder::new(&mut response)
        .set_str("Mcp-Session-Id", &session_value)?
        .set_str("Mcp-Protocol-Version", "2025-06-18")?
        .set_optional(
            "Dynmcp-Protocol-Method",
            result.respx.protocol_method.as_deref(),
        )?
        .set_str("Dynmcp-Protocol-Type", ids_metadata.proto_type.as_str())?;

    Ok(response)
}

pub async fn mcp_get(Path(ids_id): Path<String>) -> Result<impl IntoResponse, RestAPIError> {
    // TODO Last-Event-ID

    // broadcast stream for MCP Notifications
    let broadcast_tx = get_global_broadcast_tx()?;
    let ids_id: Arc<str> = ids_id.into();
    let broadcast_stream = BroadcastStream::new(broadcast_tx.subscribe())
        .filter_map({
            move |res| {
                let ids_id = ids_id.clone();
                async move {
                    match res {
                        Ok(msg) if msg.ids_id.as_str() == ids_id.as_ref() => {
                            Some(Event::default().data(msg.message))
                        }
                        Err(BroadcastStreamRecvError::Lagged(n)) => {
                            error!("iDS: {}, missed {} broadcast messages", &*ids_id, n);
                            None
                        }
                        _ => None,
                    }
                }
            }
        })
        .map(|event| Ok::<Event, Infallible>(event));

    // heartbeat stream
    let heartbeat_stream = IntervalStream::new(interval(Duration::from_secs(10)))
        .map(|_| Ok::<Event, Infallible>(Event::default().event("ping").data("keep-alive")));

    // combined stream
    let combined = select(broadcast_stream, heartbeat_stream);

    Ok(Sse::new(combined).into_response())
}
