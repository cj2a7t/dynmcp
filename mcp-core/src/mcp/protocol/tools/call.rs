use anyhow::{anyhow, Result};
use async_trait::async_trait;
use mcp_common::{
    http_client::model::HttpRequestOptions, provider::global_provider::get_http_client,
};
use mcp_macro::mcp_proto;
use serde_json::Value;

use crate::{
    mcp::protocol::mcp_protocol::{MCProtocol, Requestx, Responsex},
    model::spec::protocol::{ToolCallRequest, ToolCallResponse, ToolCallResult, ToolContent},
};

#[derive(Default)]
pub struct CallToolProtocol;

#[async_trait]
#[mcp_proto("tools/call")]
impl MCProtocol for CallToolProtocol {
    type JSONRPCRequest = ToolCallRequest;
    type JSONRPCResponse = ToolCallResponse;

    fn cast(&self, value: &Value) -> Result<Self::JSONRPCRequest> {
        Ok(serde_json::from_value(value.to_owned())?)
    }

    async fn call(
        &self,
        req: ToolCallRequest,
        reqx: &Requestx,
    ) -> Result<(ToolCallResponse, Responsex)> {
        // find tds by name
        let tds = reqx
            .mcp_cache
            .get_tds_by_name(&req.params.name)
            .ok_or_else(|| anyhow!("TDS not found for name: {}", &req.params.name))?;

        // call API
        let tds_ext_info = tds.tds_ext_info;

        // TODO only a mock call now
        // TODO the request should be constructed according to the parameter types.
        let url = format!("{}{}", tds_ext_info.domain, tds_ext_info.path);

        let toolcall_req = HttpRequestOptions::<serde_json::Value> {
            method: tds_ext_info.method.to_string(),
            headers: Some(Default::default()),
            body: Some(serde_json::Value::Null),
        };
        let toolcall_res: String = get_http_client()?
            .request_uri(&url, toolcall_req)
            .await
            .unwrap();

        let result = ToolCallResult {
            is_error: false,
            content: vec![ToolContent {
                content_type: "text".into(),
                text: toolcall_res,
            }],
        };

        let response = ToolCallResponse {
            id: req.id,
            jsonrpc: req.jsonrpc,
            result,
        };

        Ok((response, Responsex::default()))
    }
}
