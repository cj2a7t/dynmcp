use std::{any::type_name, sync::Arc};

use crate::datasource::datasource::DataSource;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use chrono::{NaiveDateTime, Utc};
use mcp_common::{
    cache::mcp_cache::McpCache,
    provider::global_provider::get_mysql_pool,
    xds::{ids::IDS, tds::TDS},
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use tracing::{debug, info, warn};

/// Represents a row in the `dynmcp_xds` table.
///
/// ### MySQL Table Structure
/// ```sql
/// CREATE TABLE IF NOT EXISTS dynmcp_xds (
///     id BIGINT PRIMARY KEY AUTO_INCREMENT,
///     `key` VARCHAR(255) NOT NULL UNIQUE,
///     xds_type VARCHAR(64) NOT NULL,
///     xds_json TEXT NOT NULL,
///     status ENUM('pending', 'syncing', 'synced') NOT NULL DEFAULT 'pending',
///     create_time DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
///     update_time DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
/// ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
/// ```
///
/// ### Field Descriptions
/// - **`id`**: Auto-increment primary key.
/// - **`key`**: Unique identifier for the XDS object (e.g., service ID).
/// - **`xds_type`**: Type of the XDS object (e.g., `TDS`, `IDS`, `CDS`, etc.).
/// - **`xds_json`**: Serialized JSON representation of the XDS object.
/// - **`status`**: Synchronization status: `pending`, `syncing`, or `synced`.
/// - **`create_time`**: Timestamp when the record was created.
/// - **`update_time`**: Timestamp when the record was last updated.
#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct XDSRecord {
    pub id: i64,
    pub key: String,
    pub xds_type: String,
    pub xds_json: String,
    pub status: String,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
}

pub struct MysqlDataSource {
    mcp_cache: Arc<McpCache>,
}

impl MysqlDataSource {
    pub fn new(mcp_cache: Arc<McpCache>) -> Self {
        Self { mcp_cache }
    }
    async fn insert_into_cache(&self, record: &XDSRecord) -> bool {
        match record.xds_type.as_str() {
            "TDS" => match serde_json::from_str::<TDS>(&record.xds_json) {
                Ok(tds) => {
                    self.mcp_cache.insert_tds(record.key.clone(), tds);
                    true
                }
                Err(e) => {
                    tracing::warn!("Failed to parse TDS from record {}: {}", record.key, e);
                    false
                }
            },
            "IDS" => match serde_json::from_str::<IDS>(&record.xds_json) {
                Ok(ids) => {
                    self.mcp_cache.insert_ids(record.key.clone(), ids);
                    true
                }
                Err(e) => {
                    tracing::warn!("Failed to parse IDS from record {}: {}", record.key, e);
                    false
                }
            },
            other => {
                tracing::warn!("Unknown xds_type `{}` for key {}", other, record.key);
                false
            }
        }
    }
}

#[async_trait]
impl DataSource for MysqlDataSource {
    async fn fetch_and_watch(self: Arc<Self>) -> Result<()> {
        let pool = get_mysql_pool();
        const PAGE_SIZE: i64 = 100;

        // Initial full load
        info!("🔄 Starting initial full load of XDS records...");

        let mut offset: i64 = 0;
        loop {
            let rows: Vec<XDSRecord> = sqlx::query_as::<_, XDSRecord>(
                r#"
            SELECT id, `key`, xds_type, xds_json, status, create_time, update_time
            FROM dynmcp_xds
            ORDER BY id ASC
            LIMIT ? OFFSET ?
            "#,
            )
            .bind(PAGE_SIZE)
            .bind(offset)
            .fetch_all(&*pool)
            .await?;

            if rows.is_empty() {
                info!("✅ Initial load completed.");
                break;
            }

            debug!("📄 Loaded {} records (offset = {})", rows.len(), offset);

            for record in &rows {
                debug!(
                    "📥 Inserting record into cache: id = {}, key = {}",
                    record.id, record.key
                );
                self.insert_into_cache(record).await;
            }

            offset += PAGE_SIZE;
        }

        // Watch for pending
        info!("👀 Entering watch loop for pending XDS records...");

        loop {
            let pending_rows: Vec<XDSRecord> = sqlx::query_as::<_, XDSRecord>(
                r#"
            SELECT id, `key`, xds_type, xds_json, status, create_time, update_time
            FROM dynmcp_xds
            WHERE status = 'pending'
            ORDER BY id ASC
            LIMIT ?
            "#,
            )
            .bind(PAGE_SIZE)
            .fetch_all(&*pool)
            .await?;

            if pending_rows.is_empty() {
                debug!("⏸ No pending records found, sleeping 5s...");
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                continue;
            }

            info!("🔄 Found {} pending records", pending_rows.len());

            for record in &pending_rows {
                debug!("🔃 Marking record as syncing: id = {}", record.id);
                sqlx::query("UPDATE dynmcp_xds SET status = 'syncing' WHERE id = ?")
                    .bind(record.id)
                    .execute(&*pool)
                    .await?;
            }

            for record in pending_rows {
                debug!(
                    "📥 Syncing record into cache: id = {}, key = {}",
                    record.id, record.key
                );
                let ok = self.insert_into_cache(&record).await;
                let new_status = if ok { "synced" } else { "pending" };

                if ok {
                    info!("✅ Synced record: id = {}, key = {}", record.id, record.key);
                } else {
                    warn!(
                        "⚠️ Failed to sync record, resetting to pending: id = {}, key = {}",
                        record.id, record.key
                    );
                }

                sqlx::query("UPDATE dynmcp_xds SET status = ? WHERE id = ?")
                    .bind(new_status)
                    .bind(record.id)
                    .execute(&*pool)
                    .await?;
            }
        }
    }

    async fn put<T>(self: Arc<Self>, id: &str, value: &T) -> Result<T>
    where
        T: serde::Serialize + Clone + Send + Sync + 'static,
    {
        let pool = get_mysql_pool();
        let value_json = serde_json::to_string(value)?;

        let xds_type = std::any::type_name::<T>()
            .rsplit("::")
            .next()
            .unwrap_or("Unknown");
        let now = Utc::now().naive_utc();

        sqlx::query(
            r#"
            INSERT INTO dynmcp_xds (`key`, xds_type, xds_json, create_time, update_time, status)
            VALUES (?, ?, ?, ?, ?, 'pending')
            ON DUPLICATE KEY UPDATE
                xds_type = VALUES(xds_type),
                xds_json = VALUES(xds_json),
                update_time = VALUES(update_time)
        "#,
        )
        .bind(id)
        .bind(xds_type)
        .bind(&value_json)
        .bind(now)
        .bind(now)
        .execute(&*pool)
        .await?;

        Ok(value.clone())
    }

    async fn get<T>(self: Arc<Self>, id: &str) -> Result<T>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        let pool = get_mysql_pool();

        let record: Option<XDSRecord> =
            sqlx::query_as::<_, XDSRecord>("SELECT id, `key`, xds_type, xds_json, status, create_time, update_time FROM dynmcp_xds WHERE `key` = ?")
                .bind(id)
                .fetch_optional(&*pool)
                .await?;

        match record {
            Some(row) => {
                let xds = serde_json::from_str::<T>(&row.xds_json)
                    .map_err(|e| anyhow!("Failed to deserialize xds_json: {}", e))?;
                Ok(xds)
            }
            None => Err(anyhow!("No record found for key `{}`", id)),
        }
    }

    async fn delete(self: Arc<Self>, id: &str) -> Result<bool> {
        let pool = get_mysql_pool();

        let record: Option<(String,)> =
            sqlx::query_as("SELECT xds_type FROM dynmcp_xds WHERE `key` = ?")
                .bind(id)
                .fetch_optional(&*pool)
                .await?;

        if let Some((_xds_type,)) = record {
            let result = sqlx::query("DELETE FROM dynmcp_xds WHERE `key` = ?")
                .bind(id)
                .execute(&*pool)
                .await?;
            Ok(result.rows_affected() > 0)
        } else {
            Ok(false)
        }
    }

    async fn get_all<T>(self: Arc<Self>) -> Result<Vec<T>>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        let full_type = type_name::<T>();
        let short_type = full_type
            .rsplit("::")
            .next()
            .ok_or_else(|| anyhow!("Failed to extract type name from: {}", full_type))?;

        let pool = get_mysql_pool();

        let rows: Vec<XDSRecord> = sqlx::query_as::<_, XDSRecord>(
            r#"
            SELECT id, `key`, xds_type, xds_json, status, create_time, update_time
            FROM dynmcp_xds
            WHERE xds_type = ?
            ORDER BY id ASC
            "#,
        )
        .bind(short_type)
        .fetch_all(&*pool)
        .await?;

        let mut result = Vec::new();
        for row in rows {
            match serde_json::from_str::<T>(&row.xds_json) {
                Ok(xds) => result.push(xds),
                Err(e) => {
                    warn!(
                        "Failed to deserialize xds_json for key {} into type {}: {}",
                        row.key, short_type, e
                    );
                }
            }
        }

        Ok(result)
    }
}
