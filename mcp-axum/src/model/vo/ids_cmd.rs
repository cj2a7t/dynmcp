use mcp_common::xds::ids::IDS;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IDSCmd {
    pub name: String,
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
