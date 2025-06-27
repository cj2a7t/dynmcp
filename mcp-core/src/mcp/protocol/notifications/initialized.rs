use anyhow::Result;
use mcp_macro::mcp_proto;
use serde_json::Value;

use crate::{
    mcp::protocol::mcp_protocol::{MCProtocol, Requestx, Responsex},
    model::spec::protocol::{NotificationsInitializedRequest, NotificationsInitializedResponse},
};

#[derive(Default)]
pub struct NotificationInitializedProtocol;

#[mcp_proto("notifications/initialized")]
impl MCProtocol for NotificationInitializedProtocol {
    type JSONRPCRequest = NotificationsInitializedRequest;
    type JSONRPCResponse = NotificationsInitializedResponse;

    fn cast(&self, value: &Value) -> Result<Self::JSONRPCRequest> {
        Ok(serde_json::from_value(value.to_owned())?)
    }

    fn call(
        &self,
        _req: NotificationsInitializedRequest,
        _reqx: &Requestx,
    ) -> (NotificationsInitializedResponse, Responsex) {
        (Default::default(), Responsex::accepted())
    }
}
