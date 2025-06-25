use axum::{ http::{ header::CONTENT_TYPE, StatusCode }, response::{ IntoResponse, Response } };
use derive_new::new;
use serde::Serialize;

#[derive(Serialize, new)]
pub struct RestAPIResponse<T> where T: Serialize {
    pub code: i32,
    #[new(into)]
    pub message: String,
    pub data: Option<T>,
}

impl<T: Serialize> RestAPIResponse<T> {
    pub fn success(data: T) -> Self {
        Self::new(200, "success", Some(data))
    }

    pub fn error(msg: &str) -> Self {
        Self::new(500, msg, None)
    }
}

impl<T> IntoResponse for RestAPIResponse<T> where T: Serialize {
    fn into_response(self) -> Response {
        match serde_json::to_string(&self) {
            Ok(body) =>
                (StatusCode::OK, [(CONTENT_TYPE, "application/json")], body).into_response(),
            Err(_) =>
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    [(CONTENT_TYPE, "text/plain")],
                    "Internal Server Error".to_string(),
                ).into_response(),
        }
    }
}
