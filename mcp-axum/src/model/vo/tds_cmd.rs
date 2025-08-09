use std::collections::HashMap;

use mcp_common::xds::tds::{TDSx, TDS};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct TDSCmd {
    #[validate(length(min = 1, message = "IDS name cannot be empty"))]
    pub name: String,
    #[validate(length(min = 1, message = "IDS tool_ids must contain at least 1 element"))]
    pub description: String,
    pub input_schema: HashMap<String, Value>,
    pub tds_ext_info: TDSx,
}

pub trait IntoTDS {
    fn into_tds(self, id: String) -> TDS;
}

impl IntoTDS for TDSCmd {
    fn into_tds(self, id: String) -> TDS {
        TDS {
            id,
            name: self.name,
            description: self.description,
            input_schema: self.input_schema,
            tds_ext_info: self.tds_ext_info,
        }
    }
}
