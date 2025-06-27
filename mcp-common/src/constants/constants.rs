pub mod mcp_protocol_consts {
    pub const JSONRPC_VERSION: &str = "2.0";
    pub const SERVER_NAME: &str = "mcprust";
    pub const SERVER_VERSION: &str = "1.0.0";
    pub const PROTOCOL_VERSION: &str = "2025-03-26";
}


pub mod mcp_cache_consts {
    pub const ETCD_TDS_PREFIX: &str = "/dynmcp/tds/";
    pub const ETCD_IDS_PREFIX: &str = "/dynmcp/ids/";
}