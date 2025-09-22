use axum::{
    extract::{Path, State},
    response::{IntoResponse, Response},
    Json,
};
use mcp_common::{enums::ids_protocol_type::IdsProtoType, xds::ids::IDSMetadata};
use mcp_core::{
    error::dyn_execute_error::DynExecuteError,
    mcp::protocol::mcp_protocol::{self, Requestx},
};
use serde_json::{from_str, Value};

use crate::{
    error::api_error::RestAPIError,
    model::{
        app_state::AppState,
        jsonrpc_response::{once_sse, JSONRpcResponse},
    },
};

pub async fn mcp_post(
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
    let response = match proto_type {
        IdsProtoType::StreamableStateless => {
            JSONRpcResponse::with_u16_status(result.respx.http_status, result.response)
                .into_response()
        }
        IdsProtoType::Other(_) => once_sse(&result.response),
    };

    Ok(response)
}
