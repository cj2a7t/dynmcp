use std::{any::TypeId, sync::Arc};

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

/// Represents a row in the `dynmcp_xds` table.
///
/// MySQL Table Definition:
/// ```sql
/// CREATE TABLE IF NOT EXISTS dynmcp_xds (
///     id BIGINT PRIMARY KEY AUTO_INCREMENT,
///     `key` VARCHAR(255) NOT NULL UNIQUE,
///     xds_type VARCHAR(64) NOT NULL,
///     xds_json TEXT NOT NULL,
///     create_time DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
///     update_time DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
/// ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
/// ```
///
/// Field meanings:
/// - `id`: Auto-increment primary key.
/// - `key`: Unique key used to identify an XDS object (e.g., service ID).
/// - `xds_type`: Type of the XDS object (e.g., TDS, IDS, CDS, etc).
/// - `xds_json`: Serialized JSON content of the XDS object.
/// - `create_time`: Time when the record was created (auto-filled by DB).
/// - `update_time`: Time when the record was last updated (auto-updated on modification).

/// XDS database record structure mapped to the `mcp_data` table.
#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct XDSRecord {
    pub id: i64,
    pub key: String,
    pub xds_type: String,
    pub xds_json: String,
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
}

#[async_trait]
impl DataSource for MysqlDataSource {
    async fn fetch_and_watch(self: Arc<Self>) -> Result<()> {
        let pool = get_mysql_pool();
        let mut offset: i64 = 0;
        const PAGE_SIZE: i64 = 100;
        loop {
            let rows: Vec<XDSRecord> = sqlx::query_as::<_, XDSRecord>(
                r#"
                SELECT * FROM mcp_data
                ORDER BY id ASC
                LIMIT ? OFFSET ?
                "#,
            )
            .bind(PAGE_SIZE)
            .bind(offset)
            .fetch_all(&*pool)
            .await?;

            if rows.is_empty() {
                break;
            }

            for record in rows {
                match record.xds_type.as_str() {
                    "TDS" => match serde_json::from_str::<TDS>(&record.xds_json) {
                        Ok(tds) => {
                            self.mcp_cache.insert_tds(record.key.clone(), tds);
                        }
                        Err(e) => {
                            tracing::warn!("Failed to parse TDS from record {}: {}", record.key, e);
                        }
                    },
                    "IDS" => match serde_json::from_str::<IDS>(&record.xds_json) {
                        Ok(ids) => {
                            self.mcp_cache.insert_ids(record.key.clone(), ids);
                        }
                        Err(e) => {
                            tracing::warn!("Failed to parse IDS from record {}: {}", record.key, e);
                        }
                    },
                    other => {
                        tracing::warn!("Unknown xds_type `{}` for key {}", other, record.key);
                    }
                }
            }

            offset += PAGE_SIZE;
        }
        Ok(())
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
            INSERT INTO mcp_data (`key`, xds_type, xds_json, create_time, update_time)
            VALUES (?, ?, ?, ?, ?)
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

        let key = id.to_string();
        if TypeId::of::<T>() == TypeId::of::<TDS>() {
            let tds: TDS = serde_json::from_str(&value_json).unwrap();
            self.mcp_cache.insert_tds(key, tds);
        } else if TypeId::of::<T>() == TypeId::of::<IDS>() {
            let ids: IDS = serde_json::from_str(&value_json).unwrap();
            self.mcp_cache.insert_ids(key, ids);
        } else {
            tracing::warn!("Unsupported type in put(): {}", xds_type);
        }

        Ok(value.clone())
    }

    async fn get<T>(self: Arc<Self>, id: &str) -> Result<T>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        let pool = get_mysql_pool();

        let record: Option<XDSRecord> =
            sqlx::query_as::<_, XDSRecord>("SELECT * FROM mcp_data WHERE `key` = ?")
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
            sqlx::query_as("SELECT xds_type FROM mcp_data WHERE `key` = ?")
                .bind(id)
                .fetch_optional(&*pool)
                .await?;

        if let Some((xds_type,)) = record {
            let result = sqlx::query("DELETE FROM mcp_data WHERE `key` = ?")
                .bind(id)
                .execute(&*pool)
                .await?;
            let key = id.to_string();
            match xds_type.as_str() {
                "TDS" => self.mcp_cache.remove_tds(&key),
                "IDS" => self.mcp_cache.remove_ids(&key),
                _ => tracing::warn!("Unknown xds_type `{}` for key {}", xds_type, key),
            }
            Ok(result.rows_affected() > 0)
        } else {
            Ok(false)
        }
    }
}
