use anyhow::Error;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug)]
pub struct RestAPIError {
    pub error: Error,
    pub status: StatusCode,
}

impl RestAPIError {
    pub fn internal<E: Into<Error>>(err: E) -> Self {
        Self {
            error: err.into(),
            status: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn not_found<E: Into<Error>>(err: E) -> Self {
        Self {
            error: err.into(),
            status: StatusCode::NOT_FOUND,
        }
    }

    pub fn bad_request<E: Into<Error>>(err: E) -> Self {
        Self {
            error: err.into(),
            status: StatusCode::BAD_REQUEST,
        }
    }

    pub fn unauthorized<E: Into<Error>>(err: E) -> Self {
    Self {
        error: err.into(),
        status: StatusCode::UNAUTHORIZED,
    }
}
}

impl<E: Into<Error>> From<E> for RestAPIError {
    fn from(err: E) -> Self {
        RestAPIError::internal(err)
    }
}

impl IntoResponse for RestAPIError {
    fn into_response(self) -> Response {
        let status = self.status;
        let body = Json(json!({
            "code": status.as_u16(),
            "error": self.error.to_string(),
        }));
        (status, body).into_response()
    }
}
