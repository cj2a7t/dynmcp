use std::{any::Any, sync::Arc};

use anyhow::Result;
use async_trait::async_trait;
use dashmap::DashMap;
use erased_serde::Serialize as ErasedSerialize;
use mcp_common::cache::mcp_cache::McpCache;
use once_cell::sync::Lazy;
use serde_json::Value;

static REGISTRY: Lazy<DashMap<String, Arc<dyn DynMCProtocol>>> = Lazy::new(DashMap::new);

#[derive(Debug)]
pub struct Responsex {
    pub http_status: u16,
}
impl Responsex {
    pub fn default() -> Self {
        Responsex { http_status: 200 }
    }
    pub fn accepted() -> Self {
        Responsex { http_status: 202 }
    }
}

pub struct Requestx<'a> {
    pub mcp_cache: &'a McpCache,
    pub instance_id: String,
}

pub struct DynExecuteResult {
    pub response: Box<dyn ErasedSerialize + Send + Sync>,
    pub respx: Responsex,
}

#[async_trait]
pub trait MCProtocol {
    type JSONRPCRequest: 'static + Send;
    type JSONRPCResponse: 'static + ErasedSerialize + Send + Sync;

    async fn call(
        &self,
        req: Self::JSONRPCRequest,
        _reqx: &Requestx,
    ) -> Result<(Self::JSONRPCResponse, Responsex)>;

    fn cast(&self, value: &Value) -> Result<Self::JSONRPCRequest>;
}

#[async_trait]
pub trait DynMCProtocol: Send + Sync {
    async fn call_boxed_erased(
        &self,
        req: Box<dyn Any + Send + Sync>,
        _reqx: &Requestx,
    ) -> Result<(Box<dyn ErasedSerialize + Send + Sync>, Responsex)>;

    fn cast_boxed(&self, value: &Value) -> Result<Box<dyn Any + Send + Sync>>;
}
pub struct DynWrapper<P: MCProtocol> {
    protocol: P,
}
#[async_trait]
impl<P> DynMCProtocol for DynWrapper<P>
where
    P: MCProtocol + Send + Sync + 'static,
    P::JSONRPCRequest: Send + Sync + 'static,
    P::JSONRPCResponse: ErasedSerialize + Send + Sync + 'static,
{
    async fn call_boxed_erased(
        &self,
        req: Box<dyn Any + Send + Sync>,
        _reqx: &Requestx,
    ) -> Result<(Box<dyn ErasedSerialize + Send + Sync>, Responsex)> {
        let req = req
            .downcast::<P::JSONRPCRequest>()
            .expect("downcast failed");
        let (response, extra) = self.protocol.call(*req, _reqx).await?;
        Ok((Box::new(response), extra))
    }

    fn cast_boxed(&self, value: &Value) -> Result<Box<dyn Any + Send + Sync>> {
        let req = self.protocol.cast(value)?;
        Ok(Box::new(req))
    }
}

pub fn register_protocol<P>(key: &str, protocol: P)
where
    P: MCProtocol + Send + Sync + 'static,
    P::JSONRPCRequest: Send + Sync + 'static,
    P::JSONRPCResponse: ErasedSerialize,
{
    REGISTRY.insert(key.to_string(), Arc::new(DynWrapper { protocol }));
}

pub fn get_protocol(method: &str) -> Option<Arc<dyn DynMCProtocol>> {
    REGISTRY
        .get(method)
        .map(|v| Arc::clone(&*v) as Arc<dyn DynMCProtocol>)
}

/// Executes a registered JSON-RPC method with a dynamically typed response.
///
/// This function is designed for dynamic dispatch contexts, such as HTTP servers,
/// where the exact response type isn't known at compile time. The response is returned as
/// a boxed `dyn ErasedSerialize`, which allows type-erased serialization (e.g., to JSON).
///
/// It also returns a `Responsex` containing metadata such as HTTP status and processing time.
///
/// # Parameters
/// - `jsonrpc_request`: A JSON value representing the JSON-RPC request. Must contain a `"method"` field.
/// - `_reqx`: A reference to the contextual `Requestx`, including shared resources like `McpCache`.
///
/// # Returns
/// - `Ok(Some(DynExecuteResult))` if the method is found and executed successfully.
/// - `Ok(None)` if the method is not found or request casting fails.
/// - `Err(_)` if an internal error occurs.
///
/// # Example
/// ```rust
/// if let Some(result) = execute_dyn(req, &cache).await? {
///     tracing::info!("Took {}ms", result.respx.elapsed_ms);
///     let json = serde_json::to_string(&result.response)?;
/// }
/// ```
pub async fn execute_dyn(
    jsonrpc_request: Value,
    _reqx: &Requestx<'_>,
) -> Result<Option<DynExecuteResult>> {
    let method = match jsonrpc_request.get("method").and_then(|v| v.as_str()) {
        Some(m) => m,
        None => return Ok(None),
    };

    let strat: Arc<dyn DynMCProtocol> = match get_protocol(method) {
        Some(s) => s,
        None => return Ok(None),
    };

    let req = match strat.cast_boxed(&jsonrpc_request) {
        Ok(r) => r,
        Err(_) => return Ok(None),
    };

    let (response, respx) = strat.call_boxed_erased(req, _reqx).await?;
    Ok(Some(DynExecuteResult { response, respx }))
}
