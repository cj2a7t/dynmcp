use anyhow::{anyhow, Result};
use std::fmt;
use std::str::FromStr;

/// Represents supported protocol methods.
///
/// Provides conversions:
/// - `as_str()` to get the string slice (`&str`)
/// - `Display` to get a `String` via `to_string()`
/// - `FromStr` to parse from a string into `ProtocolMethod`
///
/// # Examples
///
/// ```
/// use anyhow::Result;
/// use std::str::FromStr;
/// use your_crate::ProtocolMethod;
///
/// fn main() -> Result<()> {
///     // Enum to &str
///     assert_eq!(ProtocolMethod::ToolsCall.as_str(), "tools/call");
///
///     // Enum to String
///     assert_eq!(ProtocolMethod::ToolsList.to_string(), "tools/list");
///
///     // String to Enum
///     let method: ProtocolMethod = "initialize".parse()?;
///     assert_eq!(method, ProtocolMethod::Initialize);
///
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProtocolMethod {
    Initialize,
    ToolsCall,
    ToolsList,
    NotificationsInitialized,
}

impl fmt::Display for ProtocolMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ProtocolMethod::Initialize => "initialize",
            ProtocolMethod::ToolsCall => "tools/call",
            ProtocolMethod::ToolsList => "tools/list",
            ProtocolMethod::NotificationsInitialized => "notifications/initialized",
        };
        write!(f, "{}", s)
    }
}

impl ProtocolMethod {
    /// Returns the string representation of the protocol method as `&'static str`.
    pub fn as_str(&self) -> &'static str {
        match self {
            ProtocolMethod::Initialize => "initialize",
            ProtocolMethod::ToolsCall => "tools/call",
            ProtocolMethod::ToolsList => "tools/list",
            ProtocolMethod::NotificationsInitialized => "notifications/initialized",
        }
    }
}

impl FromStr for ProtocolMethod {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "initialize" => Ok(ProtocolMethod::Initialize),
            "tools/call" => Ok(ProtocolMethod::ToolsCall),
            "tools/list" => Ok(ProtocolMethod::ToolsList),
            "notifications/initialized" => Ok(ProtocolMethod::NotificationsInitialized),
            _ => Err(anyhow!("Invalid ProtocolMethod: {}", s)),
        }
    }
}
