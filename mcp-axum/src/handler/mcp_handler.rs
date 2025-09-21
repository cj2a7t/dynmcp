use axum::{
    extract::{Path, State},
    http::HeaderMap,
    response::IntoResponse,
    Json,
};
use mcp_core::{
    error::dyn_execute_error::DynExecuteError,
    mcp::protocol::mcp_protocol::{self, Requestx},
};
use serde_json::Value;
use tokio::sync::mpsc::unbounded_channel;

use crate::{
    error::api_error::RestAPIError,
    model::{app_state::AppState, jsonrpc_response::JSONRpcResponse},
};

pub async fn mcp_post(
    Path(ids_id): Path<String>,
    State(state): State<AppState>,
    Json(jsonrpc_request): Json<Value>,
) -> Result<impl IntoResponse, RestAPIError> {
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

    // execute dynamic mcp protocol
    let result = mcp_protocol::execute_dyn(jsonrpc_request, &reqx)
        .await
        .map_err(|err| RestAPIError::for_json_rpc(err))?;

    // convert the response to JSON-RPC response format
    Ok(JSONRpcResponse::with_u16_status(
        result.respx.http_status,
        result.response,
    ))
}

pub async fn mcp_get(
    State(state): State<AppState>,
    Path(ids_id): Path<String>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, RestAPIError> {
    // get the MCP session id from the headers
    let mcp_session_id = headers
        .get("Mcp-Session-Id")
        .ok_or_else(|| RestAPIError::for_json_rpc(DynExecuteError::MissingMcpSessionId))?
        .to_str()?;

    let (tx, rx) = unbounded_channel::<String>();

    let existedSession = state.session_manager.sessions.get(mcp_session_id);
    if !existedSession.is_none() {
        // TODO
        return Ok(JSONRpcResponse::with_u16_status(
            409,
            "Conflict: Only one SSE stream is allowed per session",
        ));
    }

    Ok(JSONRpcResponse::with_u16_status(200, "OK"))
}
