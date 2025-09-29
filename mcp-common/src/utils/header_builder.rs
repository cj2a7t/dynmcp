use anyhow::Result;
use axum::{
    http::{HeaderName, HeaderValue},
    response::Response,
};
use std::collections::HashMap;

/// A fluent builder for setting HTTP response headers
pub struct HeaderBuilder<'a> {
    pub response: &'a mut Response,
}

impl<'a> HeaderBuilder<'a> {
    pub fn new(response: &'a mut Response) -> Self {
        Self { response }
    }

    /// Sets a header with string value
    pub fn set_str(&mut self, name: &str, value: &str) -> Result<&mut Self> {
        let header_name = HeaderName::from_bytes(name.as_bytes())
            .map_err(|e| anyhow::anyhow!("Invalid header name '{}': {}", name, e))?;
        let header_value = HeaderValue::from_str(value)
            .map_err(|e| anyhow::anyhow!("Failed to create header value for '{}': {}", name, e))?;
        self.response
            .headers_mut()
            .insert(header_name, header_value);
        Ok(self)
    }

    /// Sets a header with optional value
    pub fn set_optional(&mut self, name: &str, value: Option<&str>) -> Result<&mut Self> {
        if let Some(val) = value {
            let header_name = HeaderName::from_bytes(name.as_bytes())
                .map_err(|e| anyhow::anyhow!("Invalid header name '{}': {}", name, e))?;
            let header_value = HeaderValue::from_str(val).map_err(|e| {
                anyhow::anyhow!("Failed to create header value for '{}': {}", name, e)
            })?;
            self.response
                .headers_mut()
                .insert(header_name, header_value);
        }
        Ok(self)
    }

    /// Sets multiple headers from a HashMap
    pub fn set_multiple<K, V>(&mut self, headers: &HashMap<K, V>) -> Result<&mut Self>
    where
        K: std::fmt::Display,
        V: std::fmt::Display,
    {
        for (name, value) in headers {
            let header_name = HeaderName::from_bytes(name.to_string().as_bytes())
                .map_err(|e| anyhow::anyhow!("Invalid header name '{}': {}", name, e))?;
            let header_value = HeaderValue::from_str(&value.to_string()).map_err(|e| {
                anyhow::anyhow!("Failed to create header value for '{}': {}", name, e)
            })?;
            self.response
                .headers_mut()
                .insert(header_name, header_value);
        }
        Ok(self)
    }
}
