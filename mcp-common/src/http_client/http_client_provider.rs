use anyhow::{anyhow, Result};
use reqwest::{Client, Method, Response, StatusCode};
use serde::{de::DeserializeOwned, Serialize};

use crate::http_client::model::{HttpRequestOptions, HttpResponseFormat, JsonResponse};

#[derive(Debug, Clone)]
pub struct HttpClientProvider {
    client: Client,
}

impl HttpClientProvider {
    pub fn new() -> Result<Self> {
        let client = Client::builder().build()?;
        Ok(Self { client })
    }

    async fn send<T: Serialize + Send + Sync>(
        &self,
        url: &str,
        options: &HttpRequestOptions<T>,
    ) -> Result<Response> {
        let method = options.method.parse::<Method>()?;
        let mut req = self.client.request(method, url);

        if let Some(headers) = &options.headers {
            for (k, v) in headers {
                req = req.header(k, v);
            }
        }

        if let Some(body) = &options.body {
            req = req.json(body);
        }

        let resp = req.send().await?;
        let status = resp.status();

        if !status.is_success() {
            let err_text = resp.text().await.unwrap_or_default();
            return Err(anyhow!("Request failed: {} - {}", status, err_text));
        }

        Ok(resp)
    }

    /// Sends an HTTP request and parses the response as a `String` or `Bytes`.
    ///
    /// Example usage (returns `String`):
    /// ```
    /// let options = HttpRequestOptions {
    ///     method: "POST".to_string(),
    ///     headers: None,
    ///     body: Some(serde_json::json!({ "hello": "world" })),
    /// };
    ///
    /// let response: String = client
    ///     .request_uri("https://httpbin.org/post", options)
    ///     .await
    ///     .unwrap();
    ///
    /// println!("{}", response); // Should contain echoed JSON
    /// ```
    ///
    /// Example usage (returns `Bytes`):
    /// ```
    /// let options = HttpRequestOptions {
    ///     method: "GET".to_string(),
    ///     headers: None,
    ///     body: None,
    /// };
    ///
    /// let response: bytes::Bytes = client
    ///     .request_uri("https://httpbin.org/bytes/8", options)
    ///     .await
    ///     .unwrap();
    ///
    /// assert_eq!(response.len(), 8);
    /// ```
    pub async fn request_uri<T, R>(
        &self,
        url: &str,
        options: HttpRequestOptions<T>,
    ) -> Result<(StatusCode, R)>
    where
        T: Serialize + Send + Sync,
        R: HttpResponseFormat + Send,
    {
        let resp = self.send(url, &options).await?;
        let status = resp.status();
        let parsed = R::from_response(resp).await?;
        Ok((status, parsed))
    }

    /// Sends an HTTP request and parses the JSON response into a struct.
    ///
    /// Example usage:
    /// ```
    /// #[derive(Deserialize, Debug)]
    /// struct HttpBinResponse {
    ///     json: Option<serde_json::Value>,
    /// }
    ///
    /// let options = HttpRequestOptions {
    ///     method: "POST".to_string(),
    ///     headers: None,
    ///     body: Some(serde_json::json!({ "foo": "bar" })),
    /// };
    ///
    /// let response: HttpBinResponse = client
    ///     .request_json("https://httpbin.org/post", options)
    ///     .await
    ///     .unwrap();
    ///
    /// assert_eq!(response.json.unwrap()["foo"], "bar");
    /// ```
    pub async fn request_json<T, R>(
        &self,
        url: &str,
        options: HttpRequestOptions<T>,
    ) -> Result<(StatusCode, R)>
    where
        T: Serialize + Send + Sync,
        R: DeserializeOwned,
    {
        let resp = self.send(url, &options).await?;
        let status = resp.status();
        let parsed = JsonResponse::from_response(resp).await?;
        Ok((status, parsed))
    }
}
