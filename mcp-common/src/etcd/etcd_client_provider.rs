use anyhow::Result;
use deadpool::managed::{Manager, Metrics, Pool, RecycleResult};
use etcd_client::{Client, ConnectOptions, DeleteOptions, GetOptions, PutOptions, WatchOptions};
use std::time::Duration;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EtcdError {
    #[error("ETCD connection error: {0}")]
    Connection(#[from] etcd_client::Error),
    #[error("Data decoding error: {0}")]
    Decode(#[from] std::string::FromUtf8Error),
    #[error("Reconnection failed: {0}")]
    Reconnect(String),
}

#[derive(Debug)]
pub enum EtcdEventType {
    Put,
    Delete,
    Unknown,
}

#[derive(Debug)]
pub struct EtcdWatchEvent {
    pub event_type: EtcdEventType,
    pub key: String,
    pub value: Option<String>,
    pub prev_value: Option<String>,
}

#[derive(Clone)]
pub struct EtcdClientProvider {
    pool: Pool<EtcdClientManager>,
}

pub struct EtcdClientManager {
    endpoints: Vec<String>,
    username: String,
    password: String,
}

impl Manager for EtcdClientManager {
    type Type = Client;
    type Error = etcd_client::Error;

    async fn create(&self) -> Result<Self::Type, Self::Error> {
        let mut opts = ConnectOptions::default()
            .with_keep_alive(Duration::from_secs(5), Duration::from_secs(10))
            .with_timeout(Duration::from_secs(3))
            .with_connect_timeout(Duration::from_secs(3));

        if !self.username.is_empty() && !self.password.is_empty() {
            opts = opts.with_user(&self.username, &self.password);
        }

        Client::connect(&self.endpoints, Some(opts)).await
    }

    async fn recycle(&self, conn: &mut Self::Type, _: &Metrics) -> RecycleResult<Self::Error> {
        conn.status().await?;
        Ok(())
    }
}

impl EtcdClientProvider {
    /// Create a new EtcdClientProvider with connection pool.
    ///
    /// # Example
    /// ```rust
    /// let etcd = EtcdClientProvider::new(vec!["http://localhost:2379".into()], "".into(), "".into()).await?;

    pub async fn new(endpoints: Vec<String>, username: String, password: String) -> Result<Self> {
        let mgr = EtcdClientManager {
            endpoints,
            username,
            password,
        };
        let pool = Pool::builder(mgr).max_size(8).build().unwrap();
        Ok(Self { pool })
    }

    /// Store a key-value pair in etcd.
    ///
    /// # Example
    /// ```rust
    /// etcd.put("/foo", "bar").await?;
    /// ```
    pub async fn put(&self, key: &str, value: &str) -> Result<()> {
        let mut client = self.pool.get().await?;
        client.put(key, value, Some(PutOptions::new())).await?;
        Ok(())
    }

    /// Retrieve a value by key.
    ///
    /// # Example
    /// ```rust
    /// let val = etcd.get("/foo").await?;
    /// ```
    pub async fn get(&self, key: &str) -> Result<Option<String>> {
        let mut client = self.pool.get().await?;
        let resp = client.get(key, Some(GetOptions::new())).await?;
        let result = resp
            .kvs()
            .iter()
            .find(|kv| kv.key() == key.as_bytes())
            .map(|kv| String::from_utf8(kv.value().to_vec()))
            .transpose()
            .map_err(EtcdError::from)?;
        Ok(result)
    }

    /// Delete by key.
    ///
    /// # Example
    /// ```rust
    /// let val = etcd.delete("/foo").await?;
    /// ```
    pub async fn delete(&self, key: &str) -> Result<bool> {
        let mut client = self.pool.get().await?;
        let resp = client.delete(key, Some(DeleteOptions::new())).await?;
        Ok(resp.deleted() > 0)
    }

    /// Get all key-value pairs with a specific prefix.
    ///
    /// # Example
    /// ```rust
    /// let pairs = etcd.get_prefix("/foo/").await?;
    /// ```
    pub async fn get_prefix(&self, prefix: &str) -> Result<Vec<(String, String)>> {
        let mut client = self.pool.get().await?;
        let resp = client
            .get(prefix, Some(GetOptions::new().with_prefix()))
            .await?;
        let mut result = Vec::new();
        for kv in resp.kvs() {
            let key_str = String::from_utf8(kv.key().to_vec())?;
            if key_str == prefix {
                continue;
            }
            let value = String::from_utf8(kv.value().to_vec())?;
            result.push((key_str, value));
        }
        Ok(result)
    }

    /// Start watching a key for changes and call a callback function on updates.
    ///
    /// # Example
    /// ```rust
    /// etcd.watch("/foo", |event| {
    ///     println!("Watch event: {:?}", event);
    /// }).await?;
    /// ```
    pub async fn watch<F>(&self, key: &str, mut on_event: F) -> Result<()>
    where
        F: FnMut(EtcdWatchEvent) + Send + 'static,
    {
        let key = key.to_string();
        let pool = self.pool.clone();

        // Spawn the watcher in a background task
        tokio::spawn(async move {
            let mut delay = Duration::from_secs(1);
            loop {
                let mut client = match pool.get().await {
                    Ok(c) => c,
                    Err(_) => {
                        tokio::time::sleep(delay).await;
                        delay = std::cmp::min(delay * 2, Duration::from_secs(30));
                        continue;
                    }
                };

                let stream_result = client
                    .watch(key.as_str(), Some(WatchOptions::new().with_prev_key()))
                    .await;

                match stream_result {
                    Ok((_watcher, mut stream)) => {
                        delay = Duration::from_secs(1);
                        while let Ok(Some(resp)) = stream.message().await {
                            for event in resp.events() {
                                let event_type = match event.event_type() {
                                    etcd_client::EventType::Put => EtcdEventType::Put,
                                    etcd_client::EventType::Delete => EtcdEventType::Delete,
                                };
                                let key = event
                                    .kv()
                                    .map(|kv| String::from_utf8_lossy(kv.key()).to_string());
                                let value = event
                                    .kv()
                                    .map(|kv| String::from_utf8_lossy(kv.value()).to_string());
                                let prev_value = event
                                    .prev_kv()
                                    .map(|kv| String::from_utf8_lossy(kv.value()).to_string());

                                if let Some(k) = key {
                                    on_event(EtcdWatchEvent {
                                        event_type,
                                        key: k,
                                        value,
                                        prev_value,
                                    });
                                }
                            }
                        }
                    }
                    Err(_) => {
                        tokio::time::sleep(delay).await;
                        delay = std::cmp::min(delay * 2, Duration::from_secs(30));
                    }
                }
            }
        });

        Ok(())
    }
}
