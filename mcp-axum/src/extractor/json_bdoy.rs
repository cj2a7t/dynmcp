use anyhow::anyhow;
use axum::{
    body::Body,
    extract::{FromRequest, Json},
    http::Request,
};
use serde::de::DeserializeOwned;
use std::ops::Deref;

use crate::error::api_error::RestAPIError;

#[derive(Debug)]
pub struct ValidatedJson<T>(pub T);

impl<T> Deref for ValidatedJson<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<S, T> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Send,
    S: Send + Sync,
{
    type Rejection = RestAPIError;

    async fn from_request(req: Request<Body>, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(|err| RestAPIError::bad_request(anyhow!(err.to_string())))?;
        Ok(ValidatedJson(value))
    }
}
