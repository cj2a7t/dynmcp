use axum::response::IntoResponse;

use crate::{error::api_error::RestAPIError, model::api_response::RestAPIResponse};
use anyhow::Result;

pub async fn healthz() -> Result<impl IntoResponse, RestAPIError> {
    Ok(RestAPIResponse::success("OK"))
}
