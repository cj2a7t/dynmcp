use mcp_common::xds::tds::TDS;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProtocolRequest<T> {
    pub id: u64,
    pub method: String,
    pub jsonrpc: String,
    pub params: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProtocolNotificationRequest {
    pub method: String,
    pub jsonrpc: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProtocolEmptyRequest {
    pub id: u64,
    pub method: String,
    pub jsonrpc: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProtocolResponse<T> {
    pub id: u64,
    pub jsonrpc: String,
    pub result: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InitParams {
    #[serde(rename = "protocolVersion")]
    pub version: String,
    pub capabilities: CapabilityRequest,
    #[serde(rename = "clientInfo")]
    pub client: ClientInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InitResult {
    #[serde(rename = "protocolVersion")]
    pub version: String,
    pub capabilities: CapabilityResponse,
    #[serde(rename = "serverInfo")]
    pub server: ServerInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CapabilityRequest {
    pub resources: Value,
    pub tools: Value,
    pub prompts: Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CapabilityResponse {
    pub tools: Option<ToolCapability>,
    pub logging: Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolCapability {
    #[serde(rename = "listChanged")]
    pub list_changed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientInfo {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerInfo {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RpcResult {
    pub tools: Vec<Tool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tool {
    description: String,
    #[serde(rename = "inputSchema")]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolCallParams {
    pub name: String,
    pub arguments: HashMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolCallResult {
    #[serde(rename = "isError")]
    pub is_error: bool,
    pub content: Vec<ToolContent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolContent {
    #[serde(rename = "type")]
    pub content_type: String,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmptyParams;

pub type InitRequest = ProtocolRequest<InitParams>;
pub type InitResponse = ProtocolResponse<InitResult>;
pub type ToolCallRequest = ProtocolRequest<ToolCallParams>;
pub type ToolCallResponse = ProtocolResponse<ToolCallResult>;
pub type NotificationsInitializedRequest = ProtocolNotificationRequest;
pub type NotificationsInitializedResponse = ();
pub type ListToolsRequest = ProtocolEmptyRequest;
pub type ListToolsResponse = ProtocolResponse<RpcResult>;
