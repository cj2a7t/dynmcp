use anyhow::{anyhow, Result};
use std::time::{Duration, Instant};

use moka::future::Cache;
use once_cell::sync::OnceCell;

/// # StreamableSessionManager
///
/// A simple session manager using `moka` to store `StreamableSession`s.
/// Automatically tracks `last_active` timestamps and supports TTL (time-to-live) for sessions.
///
/// ## Initialization
///
/// ```rust
/// // Initialize the global session manager with capacity 1000 and TTL 30 minutes
/// init_session_manager(1000, Duration::from_secs(1800));
/// ```
///
/// ## Usage
///
/// ```rust
/// #[tokio::main]
/// async fn main() -> Result<()> {
///     // Create a session
///     let session = StreamableSession { ids_id: "user123".to_string() };
///
///     // Safely get the global session manager
///     let manager = get_session_manager()?;
///
///     // Insert the session
///     manager.put("sess1", &session).await;
///
///     // Retrieve the full inner session
///     if let Some(inner) = manager.get_inner("sess1").await {
///         println!("ids_id: {}", inner.session.ids_id);
///         println!("created_at: {:?}", inner.created_at);
///         println!("last_active: {:?}", inner.last_active);
///     }
///
///     // Retrieve only the business session
///     if let Some(s) = manager.get("sess1").await {
///         println!("ids_id: {}", s.ids_id);
///     }
///
///     // Remove the session
///     manager.remove("sess1").await;
///
///     Ok(())
/// }
/// ```

pub fn get_session_manager() -> Result<&'static StreamableSessionManager> {
    STREAM_SESSION_MANAGER
        .get()
        .ok_or_else(|| anyhow!("StreamableSessionManager is not initialized"))
}
pub static STREAM_SESSION_MANAGER: OnceCell<StreamableSessionManager> = OnceCell::new();
pub fn init_session_manager(max_capacity: u64, timeout: Duration) {
    STREAM_SESSION_MANAGER.get_or_init(|| StreamableSessionManager::new(max_capacity, timeout));
}

#[derive(Clone)]
pub struct StreamableSession {
    pub ids_id: String,
}

#[derive(Clone)]
pub struct InnerSession {
    pub session: StreamableSession,
    pub created_at: Instant,
    pub last_active: Instant,
}

pub struct StreamableSessionManager {
    cache: Cache<String, InnerSession>,
}

impl StreamableSessionManager {
    pub fn new(max_capacity: u64, timeout: Duration) -> Self {
        // use moka
        let cache = Cache::builder()
            .max_capacity(max_capacity)
            .time_to_live(timeout)
            .build();
        Self { cache }
    }

    pub async fn put(&self, session_id: &str, session: &StreamableSession) {
        let now = Instant::now();

        let created_at = self
            .cache
            .get(session_id)
            .await
            .map(|existing| existing.created_at)
            .unwrap_or(now);

        let inner_session = InnerSession {
            session: session.clone(),
            created_at,
            last_active: now,
        };
        self.cache
            .insert(session_id.to_string(), inner_session)
            .await;
    }

    pub async fn get_inner(&self, session_id: &str) -> Option<InnerSession> {
        let now = Instant::now();
        self.cache.get(session_id).await.map(|mut inner| {
            inner.last_active = now;
            let _ = self.cache.insert(session_id.to_string(), inner.clone());
            inner
        })
    }

    pub async fn get(&self, session_id: &str) -> Option<StreamableSession> {
        self.get_inner(session_id).await.map(|inner| inner.session)
    }

    pub async fn remove(&self, session_id: &str) {
        self.cache.invalidate(session_id).await;
    }
}
