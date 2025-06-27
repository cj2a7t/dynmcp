use crate::error::api_error::RestAPIError;
use anyhow::{Context, Result};
use mcp_common::provider::global_provider::get_etcd;

pub async fn etcd_put<T: serde::Serialize + Clone>(
    prefix: &str,
    id: &str,
    value: &T,
) -> Result<T, RestAPIError> {
    let etcd = get_etcd();
    let key = format!("{}{}", prefix, id);
    let json = serde_json::to_string(value)
        .context("Failed to serialize value")
        .map_err(|e| RestAPIError::internal(anyhow::anyhow!(e)))?;
    etcd.put(&key, &json)
        .await
        .map_err(|e| RestAPIError::internal(anyhow::anyhow!("etcd.put failed: {e}")))?;
    Ok(value.clone())
}

pub async fn etcd_get<T: for<'de> serde::Deserialize<'de>>(
    prefix: &str,
    id: &str,
) -> Result<T, RestAPIError> {
    let etcd = get_etcd();
    let key = format!("{}{}", prefix, id);
    let value = etcd
        .get(&key)
        .await
        .map_err(|e| RestAPIError::internal(anyhow::anyhow!("etcd.get failed: {e}")))?
        .ok_or_else(|| RestAPIError::not_found(anyhow::anyhow!("Key `{id}` not found")))?;

    let parsed: T = serde_json::from_str(&value)
        .context("Failed to parse JSON")
        .map_err(|e| RestAPIError::not_found(anyhow::anyhow!("Parse error for `{value}`: {e}")))?;
    Ok(parsed)
}

pub async fn etcd_delete(prefix: &str, id: &str) -> Result<bool, RestAPIError> {
    let etcd = get_etcd();
    let key = format!("{}{}", prefix, id);
    let res = etcd
        .delete(&key)
        .await
        .map_err(|e| RestAPIError::internal(anyhow::anyhow!("etcd.delete failed: {e}")))?;
    Ok(res)
}
