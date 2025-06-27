use anyhow::Result;
use mcp_common::constants::constants::mcp_protocol_consts::{
    JSONRPC_VERSION, SERVER_NAME, SERVER_VERSION,
};
use mcp_macro::mcp_proto;
use serde_json::{json, Value};

use crate::{
    mcp::protocol::mcp_protocol::{MCProtocol, Requestx, Responsex},
    model::spec::protocol::{
        CapabilityResponse, InitRequest, InitResponse, InitResult, ServerInfo, ToolCapability,
    },
};

#[derive(Default)]
pub struct InitializeProtocol;

#[mcp_proto("initialize")]
impl MCProtocol for InitializeProtocol {
    type JSONRPCRequest = InitRequest;
    type JSONRPCResponse = InitResponse;

    fn cast(&self, value: &Value) -> Result<Self::JSONRPCRequest> {
        let req = serde_json::from_value(value.to_owned())?;
        Ok(req)
    }

    fn call(&self, req: InitRequest, _reqx: &Requestx) -> (InitResponse, Responsex) {
        let response = InitResponse {
            jsonrpc: JSONRPC_VERSION.to_string(),
            id: req.id,
            result: InitResult {
                version: req.params.version,
                capabilities: CapabilityResponse {
                    tools: Some(ToolCapability {
                        list_changed: false,
                    }),
                    logging: json!({}),
                },
                server: ServerInfo {
                    name: SERVER_NAME.to_string(),
                    version: SERVER_VERSION.to_string(),
                },
            },
        };

        (response, Responsex::default())
    }
}
