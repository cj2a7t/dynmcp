use mcp_common::xds::ids::IDS;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct IDSCmd {
    #[validate(length(min = 1, message = "IDS name cannot be empty"))]
    pub name: String,
    #[validate(length(min = 1, message = "IDS tool_ids must contain at least 1 element"))]
    pub tool_ids: Vec<String>,
}

pub trait IntoIDS {
    fn into_ids(self, id: String) -> IDS;
}

impl IntoIDS for IDSCmd {
    fn into_ids(self, id: String) -> IDS {
        IDS {
            id,
            name: self.name,
            tool_ids: self.tool_ids,
        }
    }
}
