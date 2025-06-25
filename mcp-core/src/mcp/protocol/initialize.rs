use mcp_common::constants::constants::mcp_protocol_consts::{
    JSONRPC_VERSION,
    SERVER_NAME,
    SERVER_VERSION,
};
use mcp_macro::mcp_proto;
use serde::{ Deserialize, Serialize };
use serde_json::{ json, Value };
use anyhow::Result;

use crate::{
    mcp::protocol::mcp_protocol::{ MCProtocol, Requestx, Responsex },
};

#[derive(Debug, Serialize, Deserialize)]
pub struct InitializeParams {
    #[serde(rename = "protocolVersion")]
    pub protocol_version: String,
    pub capabilities: Capabilities,
    #[serde(rename = "clientInfo")]
    pub client_info: ClientInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Capabilities {
    pub resources: serde_json::Value,
    pub tools: serde_json::Value,
    pub prompts: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientInfo {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InitializeResult {
    #[serde(rename = "protocolVersion")]
    pub protocol_version: String,
    pub capabilities: ResponseCapabilities,
    #[serde(rename = "serverInfo")]
    pub server_info: ServerInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseCapabilities {
    pub tools: Option<ToolsCapability>,
    pub logging: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolsCapability {
    #[serde(rename = "listChanged")]
    pub list_changed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerInfo {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InitializeRequest {
    pub method: String,
    pub params: InitializeParams,
    pub jsonrpc: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InitializeResponse {
    pub jsonrpc: String,
    pub result: InitializeResult,
    pub id: u32,
}

#[derive(Default)]
pub struct InitializeProtocol;

#[mcp_proto("initialize")]
impl MCProtocol for InitializeProtocol {
    type JSONRPCRequest = InitializeRequest;
    type JSONRPCResponse = InitializeResponse;

    fn cast(&self, value: &Value) -> Result<Self::JSONRPCRequest> {
        let req: InitializeRequest = serde_json::from_value(value.clone())?;
        Ok(req)
    }

    fn call(
        &self,
        _req: InitializeRequest,
        _reqx: &Requestx
    ) -> (InitializeResponse, Responsex) {
        (
            InitializeResponse {
                jsonrpc: JSONRPC_VERSION.to_string(),
                id: _req.id,
                result: InitializeResult {
                    protocol_version: _req.params.protocol_version,
                    capabilities: ResponseCapabilities {
                        tools: Some(ToolsCapability {
                            list_changed: false,
                        }),
                        logging: json!({}),
                    },
                    server_info: ServerInfo {
                        name: SERVER_NAME.to_string(),
                        version: SERVER_VERSION.to_string(),
                    },
                },
            },
            Responsex::default(),
        )
    }
}
