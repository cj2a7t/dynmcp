use std::collections::HashMap;

use anyhow::Result;
use mcp_common::{constants::constants::mcp_protocol_consts::JSONRPC_VERSION, xds::tds::TDS};
use mcp_macro::mcp_proto;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    mcp::protocol::mcp_protocol::{MCProtocol, Requestx, Responsex},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct RpcResult {
    tools: Vec<Tool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tool {
    description: String,
    input_schema: HashMap<String, serde_json::Value>,
    name: String,
}

impl From<TDS> for Tool {
    fn from(tds: TDS) -> Self {
        Tool {
            description: tds.description,
            input_schema: tds.input_schema,
            name: tds.name,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct ListToolsResponse {
    jsonrpc: String,
    result: RpcResult,
    id: i32,
}

#[derive(Deserialize, Debug)]
pub struct ListToolsRequest {
    pub id: i32,
    pub method: String,
    pub jsonrpc: String,
}

#[derive(Default)]
pub struct ListToolsProtocol;

#[mcp_proto("tools/list")]
impl MCProtocol for ListToolsProtocol {
    type JSONRPCRequest = ListToolsRequest;
    type JSONRPCResponse = ListToolsResponse;

    fn cast(&self, value: &Value) -> Result<Self::JSONRPCRequest> {
        let req = serde_json::from_value(value.clone())?;
        Ok(req)
    }

    fn call(&self, req: ListToolsRequest, reqx: &Requestx) -> (ListToolsResponse, Responsex) {
        let instance_id = reqx.instance_id.clone();
        let mcp_cache = reqx.mcp_cache;

        let tools = mcp_cache
            .list_tds_by_ids_id(&instance_id)
            .into_iter()
            .map(Tool::from)
            .collect::<Vec<Tool>>();
        (
            ListToolsResponse {
                result: RpcResult { tools },
                jsonrpc: JSONRPC_VERSION.to_string(),
                id: req.id,
            },
            Responsex::default(),
        )
    }
}
