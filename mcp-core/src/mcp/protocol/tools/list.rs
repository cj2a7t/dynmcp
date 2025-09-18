use anyhow::Result;
use mcp_common::constants::constants::mcp_protocol_consts::JSONRPC_VERSION;
use mcp_macro::mcp_proto;

use crate::{
    mcp::protocol::mcp_protocol::{MCProtocol, Requestx, Responsex},
    model::spec::protocol::{ListToolsRequest, ListToolsResponse, RpcResult, Tool},
};

#[derive(Default)]
pub struct ListToolsProtocol;

#[async_trait::async_trait]
#[mcp_proto("tools/list")]
impl MCProtocol for ListToolsProtocol {
    type JSONRPCRequest = ListToolsRequest;
    type JSONRPCResponse = ListToolsResponse;

    async fn call(
        &self,
        req: ListToolsRequest,
        reqx: &Requestx,
    ) -> Result<(ListToolsResponse, Responsex)> {
        let ids_id = reqx.ids_id;
        let mcp_cache = reqx.mcp_cache;

        let tools = mcp_cache
            .list_tds_by_ids_id(ids_id)
            .into_iter()
            .map(Tool::from)
            .collect::<Vec<Tool>>();
        Ok((
            ListToolsResponse {
                result: RpcResult { tools },
                jsonrpc: JSONRPC_VERSION.to_string(),
                id: req.id,
            },
            Responsex::default(),
        ))
    }
}
