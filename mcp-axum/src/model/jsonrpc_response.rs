use axum::{
    http::{header::CONTENT_TYPE, StatusCode},
    response::{IntoResponse, Response},
};
use serde::Serialize;

pub struct JSONRpcResponse<T> {
    pub status: StatusCode,
    pub body: T,
}

impl<T> JSONRpcResponse<T> {
    pub fn new(body: T) -> Self {
        Self {
            status: StatusCode::OK,
            body,
        }
    }

    pub fn with_u16_status(u16_status: u16, body: T) -> Self {
        let status = StatusCode::from_u16(u16_status).unwrap_or(StatusCode::OK);
        Self { status, body }
    }
}

impl<T> From<T> for JSONRpcResponse<T> {
    fn from(body: T) -> Self {
        Self::new(body)
    }
}

impl<T> IntoResponse for JSONRpcResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        match serde_json::to_string(&self.body) {
            Ok(body) => (self.status, [(CONTENT_TYPE, "application/json")], body).into_response(),
            Err(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                [(CONTENT_TYPE, "text/plain")],
                "Internal Server Error".to_string(),
            )
                .into_response(),
        }
    }
}
