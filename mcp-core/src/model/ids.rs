use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IDS {
    pub id: String,
    pub name: String,
    pub tool_ids: Vec<String>,
}
