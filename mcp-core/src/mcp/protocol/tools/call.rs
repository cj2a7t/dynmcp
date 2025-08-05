use std::collections::HashMap;

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use mcp_common::{
    http_client::model::HttpRequestOptions, provider::global_provider::get_http_client,
};
use mcp_macro::mcp_proto;
use serde_json::Value;
use tracing::debug;

use crate::{
    mcp::protocol::mcp_protocol::{MCProtocol, Requestx, Responsex},
    model::spec::protocol::{ToolCallRequest, ToolCallResponse, ToolCallResult, ToolContent},
};

fn extract_required_args(
    required_params: &HashMap<String, Value>,
    args_value: Option<&Value>,
) -> Result<HashMap<String, Value>> {
    let args_map = match args_value {
        Some(Value::Object(map)) => map,
        Some(_) => {
            return Err(anyhow!("Expected object for arguments"));
        }
        None => {
            return Err(anyhow!("Missing arguments"));
        }
    };
    let mut extracted = HashMap::new();
    for key in required_params.keys() {
        if let Some(value) = args_map.get(key) {
            extracted.insert(key.clone(), value.clone());
        } else {
            return Err(anyhow!("Missing required parameter: {}", key));
        }
    }
    Ok(extracted)
}

fn build_uri_from_pattern(
    url_pattern: &str,
    path_args: &HashMap<String, Value>,
    query_args: &HashMap<String, Value>,
) -> String {
    let mut url = url_pattern.to_string();
    for (key, value) in path_args {
        let placeholder = format!("{{{}}}", key);
        url = url.replace(&placeholder, &value.as_str().unwrap_or_default());
    }
    if !query_args.is_empty() {
        let query_string = query_args
            .iter()
            .map(|(k, v)| format!("{}={}", k, v.as_str().unwrap_or_default()))
            .collect::<Vec<_>>()
            .join("&");
        url.push('?');
        url.push_str(&query_string);
    }
    url
}

#[derive(Default)]
pub struct CallToolProtocol;

#[async_trait]
#[mcp_proto("tools/call")]
impl MCProtocol for CallToolProtocol {
    type JSONRPCRequest = ToolCallRequest;
    type JSONRPCResponse = ToolCallResponse;

    async fn call(
        &self,
        req: ToolCallRequest,
        reqx: &Requestx,
    ) -> Result<(ToolCallResponse, Responsex)> {
        // 1.find tds by name
        let tds = reqx
            .mcp_cache
            .get_tds_by_name(&req.params.name)
            .ok_or_else(|| anyhow!("TDS not found for name: {}", &req.params.name))?;
        let tds_ext_info = tds.tds_ext_info;

        // 2.build request url
        // required params
        let required_params = &tds_ext_info.required_params;
        // request args
        let args = req.params.arguments;
        // path parameters
        let path_args = args.get("path");
        let extracted_path_args: HashMap<String, Value> =
            extract_required_args(&required_params, path_args)?;
        // query parameters
        let query_args = args.get("query");
        let extracted_query_args: HashMap<String, Value> =
            extract_required_args(&required_params, query_args)?;
        // build uri
        let uri = build_uri_from_pattern(
            &tds_ext_info.path,
            &extracted_path_args,
            &extracted_query_args,
        );

        // 3. call API
        // url + method + body
        let url = format!("{}{}", tds_ext_info.domain, uri);
        let method = tds_ext_info.method;
        let body = args.get("body");
        debug!("mcp_protocol[tool/call] request url: {}", url);
        debug!("mcp_protocol[tool/call] request method: {}", method);
        debug!("mcp_protocol[tool/call] request body: {:?}", body);

        let toolcall_req = HttpRequestOptions::<Value> {
            method: method,
            headers: None, // TODO: auth need rewrite
            body: body.cloned(),
        };
        let (status, toolcall_res_body) = get_http_client()?
            .request_uri(url.as_str(), toolcall_req)
            .await?;
        debug!("mcp_protocol[tool/call] response status: {}", status);
        debug!(
            "mcp_protocol[tool/call] response body: {:?}",
            toolcall_res_body
        );

        // 4. tool call result
        let result = ToolCallResult {
            is_error: !status.is_success(),
            content: vec![ToolContent {
                content_type: "text".into(),
                text: toolcall_res_body,
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
