use axum::http::HeaderMap;

/// A generic HTTP Header extractor that supports automatic conversion to multiple data types
pub struct HeaderExtractor<'a>(pub &'a HeaderMap);

impl<'a> HeaderExtractor<'a> {
    /// Creates a new HeaderExtractor instance
    ///
    /// # Example
    /// ```rust
    /// let headers = HeaderMap::new();
    /// let extractor = HeaderExtractor::new(&headers);
    /// ```
    pub fn new(headers: &'a HeaderMap) -> Self {
        Self(headers)
    }

    /// Extracts header value as string
    ///
    /// # Example
    /// ```rust
    /// let session_id = extractor.get_str("Mcp-Session-Id");
    /// // Returns: Some("abc123") or None
    /// ```
    pub fn get_str(&self, name: &str) -> Option<String> {
        self.0
            .get(name)
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string())
    }

    /// Extracts header value as numeric type
    ///
    /// # Example
    /// ```rust
    /// let timeout: Option<u64> = extractor.get_number("Timeout");
    /// let port: Option<i32> = extractor.get_number("Port");
    /// ```
    pub fn get_number<T>(&self, name: &str) -> Option<T>
    where
        T: std::str::FromStr,
    {
        self.get_str(name).and_then(|s| s.parse().ok())
    }

    /// Extracts header value as boolean
    ///
    /// # Example
    /// ```rust
    /// let debug: Option<bool> = extractor.get_bool("X-Debug");
    /// let enable_cache: Option<bool> = extractor.get_bool("Enable-Cache");
    /// ```
    pub fn get_bool(&self, name: &str) -> Option<bool> {
        self.get_str(name).and_then(|s| s.parse().ok())
    }

    /// Extracts header value as JSON deserialized type
    ///
    /// # Example
    /// ```rust
    /// #[derive(Deserialize)]
    /// struct Config { timeout: u64, debug: bool }
    ///
    /// let config: Option<Config> = extractor.get_json("X-Config");
    /// ```
    pub fn get_json<T>(&self, name: &str) -> Option<T>
    where
        T: serde::de::DeserializeOwned,
    {
        self.get_str(name)
            .and_then(|s| serde_json::from_str(&s).ok())
    }
}
