use crate::{
    mcp::protocol::mcp_protocol::{MCProtocol, Requestx, Responsex},
    model::spec::protocol::{NotificationsInitializedRequest, NotificationsInitializedResponse},
};
use anyhow::Result;
use mcp_macro::mcp_proto;

#[derive(Default)]
pub struct NotificationInitializedProtocol;

#[async_trait::async_trait]
#[mcp_proto("notifications/initialized")]
impl MCProtocol for NotificationInitializedProtocol {
    type JSONRPCRequest = NotificationsInitializedRequest;
    type JSONRPCResponse = NotificationsInitializedResponse;

    async fn call(
        &self,
        _req: NotificationsInitializedRequest,
        _reqx: &Requestx,
    ) -> Result<(NotificationsInitializedResponse, Responsex)> {
        Ok((Default::default(), Responsex::accepted()))
    }
}
