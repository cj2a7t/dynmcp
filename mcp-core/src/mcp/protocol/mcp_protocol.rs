use std::{any::Any, sync::Arc};

use anyhow::Result;
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
    pub response: Box<dyn ErasedSerialize>,
    pub respx: Responsex,
}

pub trait MCProtocol {
    type JSONRPCRequest: 'static;
    type JSONRPCResponse: ErasedSerialize + 'static;

    fn call(
        &self,
        req: Self::JSONRPCRequest,
        _reqx: &Requestx,
    ) -> (Self::JSONRPCResponse, Responsex);

    fn cast(&self, value: &Value) -> Result<Self::JSONRPCRequest>;
}

pub trait DynMCProtocol: Send + Sync {
    fn call_boxed(&self, req: Box<dyn Any>, _reqx: &Requestx) -> Box<dyn Any>;

    fn call_boxed_erased(
        &self,
        req: Box<dyn Any>,
        _reqx: &Requestx,
    ) -> (Box<dyn ErasedSerialize>, Responsex);

    fn cast_boxed(&self, value: &Value) -> Result<Box<dyn Any>>;
}

pub struct DynWrapper<P: MCProtocol> {
    protocol: P,
}

impl<P> DynMCProtocol for DynWrapper<P>
where
    P: MCProtocol + Send + Sync + 'static,
    P::JSONRPCResponse: ErasedSerialize,
{
    fn call_boxed_erased(
        &self,
        req: Box<dyn Any>,
        _reqx: &Requestx,
    ) -> (Box<dyn ErasedSerialize>, Responsex) {
        let req = req
            .downcast::<P::JSONRPCRequest>()
            .expect("req downcast failed");
        let (response, extra) = self.protocol.call(*req, _reqx);
        (Box::new(response), extra)
    }

    fn call_boxed(&self, req: Box<dyn Any>, _reqx: &Requestx) -> Box<dyn Any> {
        let req = req
            .downcast::<P::JSONRPCRequest>()
            .expect("invalid req type");
        let output = self.protocol.call(*req, _reqx);
        Box::new(output)
    }

    fn cast_boxed(&self, value: &Value) -> Result<Box<dyn Any>> {
        let req = self.protocol.cast(value)?;
        Ok(Box::new(req))
    }
}

pub fn register_protocol<P>(key: &str, protocol: P)
where
    P: MCProtocol + Send + Sync + 'static,
    P::JSONRPCResponse: ErasedSerialize,
{
    REGISTRY.insert(key.to_string(), Arc::new(DynWrapper { protocol }));
}

pub fn get_protocol(method: &str) -> Option<Arc<dyn DynMCProtocol>> {
    REGISTRY.get(method).map(|v| Arc::clone(&*v))
}

/// Executes a registered JSON-RPC method with statically known response type.
///
/// This function is used when the caller knows the expected type `O` of the response at compile time.
/// It performs the following steps:
/// 1. Extracts the `method` field from the `jsonrpc_request`.
/// 2. Looks up the corresponding `MCProtocol` implementation.
/// 3. Casts the request into the expected request type.
/// 4. Calls the protocol's handler.
/// 5. Downcasts the output to `(O, ExtraResponse)` and returns it.
///
/// # Type Parameters
/// - `O`: The expected response type. Must be `'static` and match the actual implementation.
///
/// # Parameters
/// - `jsonrpc_request`: The JSON-RPC request body as a `serde_json::Value`.
/// - `mcp_cache`: A shared reference to the `McpCache` used by the handler.
///
/// # Returns
/// - `Some((response, extra))` if everything succeeds and the output type matches `O`.
/// - `None` if any step fails (e.g., missing method, cast failure, downcast failure).
///
/// # Example
/// ```rust
/// let (resp, extra): (MyResponseType, ExtraResponse) = execute::<MyResponseType>(req, &cache)?.into();
/// ```
pub fn execute<O>(jsonrpc_request: Value, _reqx: &Requestx) -> Option<(O, Responsex)>
where
    O: 'static,
{
    let method = jsonrpc_request.get("method")?.as_str()?;
    let strat = get_protocol(method)?;
    let req = strat.cast_boxed(&jsonrpc_request).ok()?;
    let output = strat.call_boxed(req, _reqx);
    output.downcast::<(O, Responsex)>().ok().map(|b| *b)
}

/// Executes a registered JSON-RPC method with a dynamically typed response.
///
/// This function is intended for generic or dynamic dispatch contexts, such as HTTP servers,
/// where the exact response type is not known at compile time. The response is wrapped in a
/// `Box<dyn ErasedSerialize>` to allow flexible serialization (e.g., to JSON) without
/// requiring a concrete type.
///
/// It also returns an `ExtraResponse` alongside the result, which contains metadata such as
/// HTTP status code and elapsed processing time.
///
/// # Parameters
/// - `jsonrpc_request`: The JSON-RPC request body as a `serde_json::Value`. Must include a `"method"` field.
/// - `mcp_cache`: A shared reference to the in-memory `McpCache` for protocol use.
///
/// # Returns
/// - `Some(DynExecuteResult)` if:
///   - the method exists in the registry,
///   - request deserialization (`cast_boxed`) succeeds,
///   - and the handler completes normally.
/// - `None` if the method is missing, or casting fails.
///
/// # Example
/// ```rust
/// let result = execute_dyn(req, &cache)?;
/// tracing::info!("Response took {}ms", result.extra.elapsed_ms);
/// let json = serde_json::to_string(&result.response).unwrap();
/// ```

pub fn execute_dyn(jsonrpc_request: Value, _reqx: &Requestx) -> Option<DynExecuteResult> {
    let method = jsonrpc_request.get("method")?.as_str()?;
    let strat = get_protocol(method)?;
    let req = strat.cast_boxed(&jsonrpc_request).ok()?;
    let (response, respx) = strat.call_boxed_erased(req, _reqx);
    Some(DynExecuteResult { response, respx })
}
