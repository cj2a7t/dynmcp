use anyhow::Result;
use axum::{
    http::{HeaderName, HeaderValue},
    response::Response,
};

/// A fluent builder for setting HTTP response headers
pub struct HeaderBuilder<'a> {
    pub response: &'a mut Response,
}

impl<'a> HeaderBuilder<'a> {
    /// Creates a new HeaderBuilder instance
    /// # Examples
    /// ```rust
    /// let builder = HeaderBuilder::new(&mut response);
    /// ```
    pub fn new(response: &'a mut Response) -> Self {
        Self { response }
    }

    /// Sets a header with string value
    /// # Examples
    /// ```rust
    /// HeaderBuilder::new(&mut response)
    ///     .set_str("Content-Type", "application/json")?;
    ///
    /// HeaderBuilder::new(&mut response)
    ///     .set_str("Mcp-Session-Id", "abc123")?;
    /// ```
    pub fn set_str(self, name: &str, value: &str) -> Result<Self> {
        let header_name = HeaderName::from_bytes(name.as_bytes())
            .map_err(|e| anyhow::anyhow!("Invalid header name '{}': {}", name, e))?;
        let header_value = HeaderValue::from_str(value)
            .map_err(|e| anyhow::anyhow!("Failed to create header value for '{}': {}", name, e))?;
        self.response
            .headers_mut()
            .insert(header_name, header_value);
        Ok(self)
    }

    /// Sets a header with optional value (only if Some)
    /// # Examples
    /// ```rust
    /// let session_id: Option<String> = get_session_id();
    /// HeaderBuilder::new(&mut response)
    ///     .set_optional("X-Session-Id", session_id.as_deref())?;
    ///
    /// HeaderBuilder::new(&mut response)
    ///     .set_optional("X-Custom-Header", custom_value.as_deref())?;
    /// ```
    pub fn set_optional(self, name: &str, value: Option<&str>) -> Result<Self> {
        if let Some(val) = value {
            let header_value = HeaderValue::from_str(val).map_err(|e| {
                anyhow::anyhow!("Failed to create header value for '{}': {}", val, e)
            })?;
            let header_name = HeaderName::from_bytes(name.as_bytes())
                .map_err(|e| anyhow::anyhow!("Invalid header name '{}': {}", name, e))?;
            self.response
                .headers_mut()
                .insert(header_name, header_value);
        }
        Ok(self)
    }

    /// Sets multiple headers from a HashMap
    /// # Examples
    /// ```rust
    /// let mut headers = HashMap::new();
    /// headers.insert("Cache-Control", "no-cache");
    /// headers.insert("X-Custom", "value");
    /// headers.insert("Content-Type", "application/json");
    ///
    /// HeaderBuilder::new(&mut response)
    ///     .set_multiple(&headers)?;
    /// ```
    pub fn set_multiple<K, V>(self, headers: &std::collections::HashMap<K, V>) -> Result<Self>
    where
        K: std::fmt::Display,
        V: std::fmt::Display,
    {
        for (name, value) in headers {
            let header_value = HeaderValue::from_str(&value.to_string()).map_err(|e| {
                anyhow::anyhow!("Failed to create header value for '{}': {}", name, e)
            })?;
            let header_name = HeaderName::from_bytes(name.to_string().as_bytes())
                .map_err(|e| anyhow::anyhow!("Invalid header name '{}': {}", name, e))?;
            self.response
                .headers_mut()
                .insert(header_name, header_value);
        }
        Ok(self)
    }
}
