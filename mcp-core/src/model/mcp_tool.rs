use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolExtInfo {
    /// The unique ID of the tool
    pub tool_id: String,
    /// The source type of the API (e.g., 1 = internal, 2 = external)
    pub api_source: u32,
    /// A map of required parameter names and their descriptions/types
    pub required_params: HashMap<String, Value>,
    /// The API path, e.g. "/v1/email"
    pub path: String,
    /// The HTTP method, e.g. "GET" or "POST" d
    pub method: String,
    /// The base domain of the API, e.g. "api.example.com"
    pub domain: String,
    /// Configuration related to JWT authentication (empty if none)
    pub jwt_config: Option<HashMap<String, Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpTool {
    /// The name of the tool, e.g. "get_email_a30"
    pub name: String,
    /// A brief description of the tool
    pub description: String,
    /// The expected input schema for the tool
    pub input_schema: HashMap<String, Value>,
    /// Extended information about the tool's API integration
    pub tool_ext_info: ToolExtInfo,
}
