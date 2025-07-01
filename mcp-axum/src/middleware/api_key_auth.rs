use anyhow::{anyhow, Result};
use axum::{extract::FromRequestParts, http::request::Parts};

use crate::{error::api_error::RestAPIError, model::app_state::AppState};

const API_KEY_HEADER: &str = "x-api-key";

pub struct ApiKey(pub String);

impl<S> FromRequestParts<S> for ApiKey
where
    S: Send + Sync + AsRef<AppState>,
{
    type Rejection = RestAPIError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let config = state.as_ref().config.clone();
        let expected_api_key = &config.api_key;
        let actual_key = parts
            .headers
            .get(API_KEY_HEADER)
            .and_then(|v| v.to_str().ok());

        match actual_key {
            Some(key) if key == expected_api_key => Ok(ApiKey(key.to_string())),
            _ => Err(RestAPIError::unauthorized(anyhow!(
                "Invalid or missing API key"
            ))),
        }
    }
}
