use mcp_macro::mcp_proto;
use serde::{ Deserialize, Serialize };
use serde_json::{ Value };
use anyhow::Result;

use crate::mcp::protocol::mcp_protocol::{ MCProtocol, Requestx, Responsex };

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationsInitializedResponse;

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationsInitializedRequest {
    pub method: String,
    pub jsonrpc: String,
}

#[derive(Default)]
pub struct InitializeProtocol;

#[mcp_proto("notifications/initialized")]
impl MCProtocol for InitializeProtocol {
    type JSONRPCRequest = NotificationsInitializedRequest;
    type JSONRPCResponse = NotificationsInitializedResponse;

    fn cast(&self, value: &Value) -> Result<Self::JSONRPCRequest> {
        let req: NotificationsInitializedRequest = serde_json::from_value(value.clone())?;
        Ok(req)
    }

    fn call(
        &self,
        _req: NotificationsInitializedRequest,
        _reqx: &Requestx
    ) -> (NotificationsInitializedResponse, Responsex) {
        (NotificationsInitializedResponse {}, Responsex { http_status: 202 })
    }
}
