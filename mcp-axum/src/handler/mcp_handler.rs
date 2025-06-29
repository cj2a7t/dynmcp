use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use erased_serde::Serialize as ErasedSerialize;
use mcp_core::mcp::protocol::mcp_protocol::{self, Requestx};
use serde_json::Value;

use crate::model::{
    api_response::RestAPIResponse, app_state::AppState, jsonrpc_response::JSONRpcResponse,
};

pub async fn handle_message(
    Path(instance_id): Path<String>,
    State(state): State<AppState>,
    Json(jsonrpc_request): Json<Value>,
) -> impl IntoResponse {
    // The jsonrpc_request only contains information from the request body.
    // If you want to pass through specific protocol fields, please use reqx.
    let reqx = Requestx {
        mcp_cache: &state.mcp_cache,
        instance_id: instance_id.clone(),
    };

    // execute dynamic mcp protocol
    match mcp_protocol::execute_dyn(jsonrpc_request, &reqx).await {
        Ok(Some(result)) => {
            let status = StatusCode::from_u16(result.respx.http_status).unwrap_or(StatusCode::OK);
            JSONRpcResponse::with_status(status, result.response)
        }
        Ok(None) => {
            let error = RestAPIResponse::<()>::error("Invalid method or params");
            JSONRpcResponse::with_status(
                StatusCode::BAD_REQUEST,
                Box::new(error) as Box<dyn ErasedSerialize + Send + Sync>,
            )
        }
        Err(err) => {
            let error = RestAPIResponse::<()>::error(&format!("Internal error: {err}"));
            JSONRpcResponse::with_status(
                StatusCode::INTERNAL_SERVER_ERROR,
                Box::new(error) as Box<dyn ErasedSerialize + Send + Sync>,
            )
        }
    }
}
