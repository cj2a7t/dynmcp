use std::{any::Any, sync::Arc};

use anyhow::Result;
use async_trait::async_trait;
use dashmap::DashMap;
use erased_serde::Serialize as ErasedSerialize;
use mcp_common::cache::mcp_cache::McpCache;
use once_cell::sync::Lazy;
use serde_json::Value;

use crate::error::dyn_execute_error::DynExecuteError;

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
/// where the exact response type isn't known at compile time. It returns a dynamically
/// dispatched boxed response (`dyn ErasedSerialize`) along with a `Responsex` object
/// that contains metadata such as HTTP status and processing time.
///
/// The request must include a `"method"` field to route to the appropriate protocol implementation.
///
/// # Parameters
/// - `jsonrpc_request`: A JSON value representing the JSON-RPC request. Must contain a `"method"` field.
/// - `_reqx`: A reference to contextual information (`Requestx`), such as shared resources like `McpCache`.
///
/// # Returns
/// - `Ok(DynExecuteResult)` if the method is found, the request is valid, and the protocol call succeeds.
/// - `Err(DynExecuteError)` if the method is missing, unsupported, the request format is invalid,
///   or an internal error occurs during execution.
///
/// # Errors
/// Returns a [`DynExecuteError`] with appropriate status and message when:
/// - The `"method"` field is missing.
/// - The method is unsupported (not registered).
/// - The request format doesn't match the protocol's expected input.
/// - The underlying protocol execution fails.
/// ```
pub async fn execute_dyn(
    jsonrpc_request: Value,
    _reqx: &Requestx<'_>,
) -> Result<DynExecuteResult, DynExecuteError> {
    // exectract the method from the JSON-RPC request
    let method = jsonrpc_request
        .get("method")
        .and_then(|v| v.as_str())
        .ok_or(DynExecuteError::MissingMethod)?;

    // get the protocol strategy based on the method
    let strat: Arc<dyn DynMCProtocol> = get_protocol(method)
        .ok_or_else(|| DynExecuteError::UnsupportedMethod(method.to_string()))?;

    // cast the JSON-RPC request to the protocol's request type
    let req = strat
        .cast_boxed(&jsonrpc_request)
        .map_err(|_| DynExecuteError::InvalidRequest)?;

    // call the protocol's method with the request
    let (response, respx) = strat
        .call_boxed_erased(req, _reqx)
        .await
        .map_err(DynExecuteError::ExecutionError)?;

    Ok(DynExecuteResult { response, respx })
}
