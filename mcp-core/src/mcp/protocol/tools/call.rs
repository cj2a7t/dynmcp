use std::collections::HashMap;

use anyhow::Result;
use mcp_macro::mcp_proto;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::mcp::protocol::mcp_protocol::{MCProtocol, Requestx, Responsex};

#[derive(Debug, Serialize, Deserialize)]
pub struct Params {
    pub name: String,
    pub arguments: HashMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResultContent {
    #[serde(rename = "isError")]
    pub is_error: bool,
    pub content: Vec<ContentItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContentItem {
    #[serde(rename = "type")]
    pub content_type: String,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallToolRequest {
    pub method: String,
    pub params: Params,
    pub jsonrpc: String,
    pub id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallToolResponse {
    pub jsonrpc: String,
    pub result: ResultContent,
    pub id: u64,
}

#[derive(Default)]
pub struct CallToolProtocol;

#[mcp_proto("tools/call")]
impl MCProtocol for CallToolProtocol {
    type JSONRPCRequest = CallToolRequest;
    type JSONRPCResponse = CallToolResponse;

    fn cast(&self, value: &Value) -> Result<Self::JSONRPCRequest> {
        let req = serde_json::from_value(value.clone())?;
        Ok(req)
    }

    fn call(&self, req: CallToolRequest, _reqx: &Requestx) -> (CallToolResponse, Responsex) {
        (
            CallToolResponse {
                id: req.id,
                jsonrpc: req.jsonrpc,
                result: ResultContent {
                    is_error: false,
                    content: vec![ContentItem {
                        content_type: "text".to_string(),
                        text: format!(
                            "Called tool '{}' with arguments: {:?}",
                            req.method, req.params.arguments
                        ),
                    }],
                },
            },
            Responsex::default(),
        )
    }
}
