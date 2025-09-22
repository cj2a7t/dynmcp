use anyhow::Error as AnyhowError;
use mcp_common::model::http_status::DynMCPHttpStatus;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DynExecuteError {
    #[error("iDS not found")]
    IdsNotFound,

    #[error("Missing 'Mcp-Session-Id' field in headers")]
    MissingMcpSessionId,

    #[error("Missing 'method' field in request")]
    MissingMethod,

    #[error("Unsupported method: {0}")]
    UnsupportedMethod(String),

    #[error("Invalid request format")]
    InvalidRequest,

    #[error("Execution error: {0}")]
    ExecutionError(#[from] AnyhowError),
}

impl DynExecuteError {
    pub fn status(&self) -> DynMCPHttpStatus {
        match self {
            DynExecuteError::IdsNotFound => DynMCPHttpStatus::NotFound,
            DynExecuteError::MissingMcpSessionId => DynMCPHttpStatus::NotFound,
            DynExecuteError::MissingMethod => DynMCPHttpStatus::BadRequest,
            DynExecuteError::UnsupportedMethod(_) => DynMCPHttpStatus::NotFound,
            DynExecuteError::InvalidRequest => DynMCPHttpStatus::BadRequest,
            DynExecuteError::ExecutionError(_) => DynMCPHttpStatus::InternalServerError,
        }
    }

    pub fn message(&self) -> String {
        self.to_string()
    }
}
