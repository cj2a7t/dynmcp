use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProtocolRequest<T> {
    pub method: String,
    pub jsonrpc: String,
    pub id: u64,
    pub params: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProtocolResponse<T> {
    pub jsonrpc: String,
    pub id: u64,
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
pub type NotificationsInitializedRequest = ProtocolRequest<EmptyParams>;
pub type NotificationsInitializedResponse = ();
