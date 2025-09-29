use std::{convert::Infallible, sync::Arc, time::Duration};

use axum::{
    extract::{Path, State},
    http::HeaderMap,
    response::{sse::Event, IntoResponse, Response, Sse},
    Json,
};
use futures::{stream::select, StreamExt};
use mcp_common::{
    enums::ids_protocol_type::IdsProtoType,
    sse::{
        broadcast::get_global_broadcast_tx,
        session_manager::{get_session_manager, StreamableSession},
    },
    utils::{header_builder::HeaderBuilder, header_extractor::HeaderExtractor},
    xds::ids::IDSMetadata,
};
use mcp_core::{
    error::dyn_execute_error::DynExecuteError,
    mcp::protocol::mcp_protocol::{self, Requestx},
    model::spec::protocol_method::ProtocolMethod,
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
    let ids_metadata: IDSMetadata = from_str(ids.metadata.as_str())?;

    // [Core] Execute dynamic mcp protocol
    let result = mcp_protocol::execute_dyn(jsonrpc_request, &reqx)
        .await
        .map_err(|err| RestAPIError::for_json_rpc(err))?;

    // Response by ids protocol type
    let proto_type: IdsProtoType = ids_metadata.proto_type.as_str().into();
    let mut response = match proto_type {
        IdsProtoType::StreamableStateless => {
            JSONRpcResponse::with_u16_status(result.respx.http_status, result.response)
                .into_response()
        }
        _ => once_sse(&result.response),
    };

    // Responsex
    let resp_protocol_method = result
        .respx
        .protocol_method
        .ok_or_else(|| RestAPIError::for_json_rpc(DynExecuteError::MissingMethod))?;
    let proto_method = resp_protocol_method.as_str();

    // Extract header
    let header_extractor = HeaderExtractor::new(&headers);
    let session_id = header_extractor.get_str("Mcp-Session-Id");

    // Build header
    let mut header_builder = HeaderBuilder::new(&mut response);
    header_builder
        .set_str("Mcp-Protocol-Version", "2025-06-18")?
        .set_optional("Dynmcp-Protocol-Method", Some(proto_method))?
        .set_str("Dynmcp-Protocol-Type", ids_metadata.proto_type.as_str())?;

    // Verify Mcp-Session-Id
    let init_proto_method = ProtocolMethod::Initialize.as_str();
    if proto_method != init_proto_method
        && session_id.is_none()
        && proto_type == IdsProtoType::StreamableStateless
    {
        return Err(RestAPIError::for_json_rpc(
            DynExecuteError::MissingMcpSessionId,
        ));
    }

    // Use HeaderExtractor to get session ID
    let resp_session_id = result
        .respx
        .initialize_session_id
        .or_else(|| session_id)
        .unwrap_or_default();
    header_builder.set_str("Mcp-Session-Id", &resp_session_id)?;

    // Put session to global streamable session manager
    if proto_method == init_proto_method && proto_type == IdsProtoType::StreamableStateful {
        // session manager
        let session_manager = get_session_manager()?;
        let session_value = StreamableSession { ids_id: ids_id };
        session_manager.put(&resp_session_id, &session_value).await;
    }

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
