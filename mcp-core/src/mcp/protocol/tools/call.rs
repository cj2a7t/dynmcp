use anyhow::Result;
use mcp_macro::mcp_proto;
use serde_json::Value;

use crate::{
    mcp::protocol::mcp_protocol::{MCProtocol, Requestx, Responsex},
    model::spec::protocol::{ToolCallRequest, ToolCallResponse, ToolCallResult, ToolContent},
};

#[derive(Default)]
pub struct CallToolProtocol;

#[mcp_proto("tools/call")]
impl MCProtocol for CallToolProtocol {
    type JSONRPCRequest = ToolCallRequest;
    type JSONRPCResponse = ToolCallResponse;

    fn cast(&self, value: &Value) -> Result<Self::JSONRPCRequest> {
        Ok(serde_json::from_value(value.to_owned())?)
    }

    fn call(&self, req: ToolCallRequest, _reqx: &Requestx) -> (ToolCallResponse, Responsex) {
        let result = ToolCallResult {
            is_error: false,
            content: vec![ToolContent {
                content_type: "text".into(),
                text: format!(
                    "Called tool '{}' with arguments: {:?}",
                    req.params.name, req.params.arguments
                ),
            }],
        };

        let response = ToolCallResponse {
            id: req.id,
            jsonrpc: req.jsonrpc,
            result,
        };

        (response, Responsex::default())
    }
}
