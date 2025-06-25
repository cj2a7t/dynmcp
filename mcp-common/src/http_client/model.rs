use std::collections::HashMap;
use anyhow::{ Result };

use async_trait::async_trait;
use bytes::Bytes;
use reqwest::Response;
use serde::{de::DeserializeOwned, Serialize};


#[derive(Debug, Clone)]
pub struct HttpRequestOptions<T> where T: Serialize {
    pub method: String,
    pub headers: Option<HashMap<String, String>>,
    pub body: Option<T>,
}

#[async_trait]
pub trait HttpResponseFormat: Sized {
    async fn from_response(resp: Response) -> Result<Self>;
}

#[async_trait]
impl HttpResponseFormat for String {
    async fn from_response(resp: Response) -> Result<Self> {
        Ok(resp.text().await?)
    }
}

#[async_trait]
impl HttpResponseFormat for Bytes {
    async fn from_response(resp: Response) -> Result<Self> {
        Ok(resp.bytes().await?)
    }
}

pub struct JsonResponse;
impl JsonResponse {
    pub async fn from_response<R: DeserializeOwned>(resp: Response) -> Result<R> {
        Ok(resp.json::<R>().await?)
    }
}