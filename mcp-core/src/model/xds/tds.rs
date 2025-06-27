use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TDSx {
    // The base domain of the API, e.g. "api.example.com"
    pub domain: String,
    // The HTTP method, e.g. "GET" or "POST" d
    pub method: String,
    // The API path, e.g. "/v1/emails/:email_id"
    pub path: String,
    // path and query parameters that are required for the API call
    pub required_params: HashMap<String, Value>,
    // ext information about the API, such as authentication details
    pub ext_info: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TDS {
    // The unique ID of the tool
    pub id: String,
    // The name of the tool, e.g. "get_email_a30"
    pub name: String,
    // A brief description of the tool
    pub description: String,
    // The expected input schema for the tool
    pub input_schema: HashMap<String, Value>,
    // Extended information about the tool's API integration
    pub tds_ext_info: TDSx,
}

impl TDS {
    pub fn validate(&self) -> Result<()> {
        if self.id.is_empty() {
            Err(anyhow!("TDS validation failed: id is empty"))
        } else {
            Ok(())
        }
    }
}
