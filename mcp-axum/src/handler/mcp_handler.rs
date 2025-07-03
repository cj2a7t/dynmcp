use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use mcp_core::mcp::protocol::mcp_protocol::{self, Requestx};
use serde_json::Value;

use crate::{
    error::api_error::RestAPIError,
    model::{app_state::AppState, jsonrpc_response::JSONRpcResponse},
};

pub async fn handle_message(
    Path(instance_id): Path<String>,
    State(state): State<AppState>,
    Json(jsonrpc_request): Json<Value>,
) -> Result<impl IntoResponse, RestAPIError> {
    // create a request context for the MCP protocol
    let reqx = Requestx {
        mcp_cache: &state.mcp_cache,
        instance_id: instance_id.clone(),
    };

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
