use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IDS {
    pub id: String,
    pub name: String,
    pub tool_ids: Vec<String>,
}

impl IDS {
    pub fn validate(&self) -> Result<()> {
        if self.id.is_empty() {
            Err(anyhow!("IDS validation failed: id is empty"))
        } else {
            Ok(())
        }
    }
}
