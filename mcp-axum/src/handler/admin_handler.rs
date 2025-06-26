use anyhow::{Context, Result};
use axum::{extract::Path, response::IntoResponse};
use mcp_common::provider::global_provider::get_etcd;
use mcp_core::model::tds::TDS;

use crate::{error::app_error::AppError, model::api_response::RestAPIResponse};

pub async fn handle_put_tds() -> impl IntoResponse {}

pub async fn handle_get_tds(Path(tds_id): Path<String>) -> Result<impl IntoResponse, AppError> {
    let etcd = get_etcd();
    let value = etcd
        .get(&tds_id)
        .await
        .map_err(|e| AppError::internal(anyhow::anyhow!("etcd.get failed: {e}")))?
        .ok_or_else(|| AppError::not_found(anyhow::anyhow!("TDS `{tds_id}` not found")))?;

    let tds: TDS = serde_json::from_str(&value)
        .context("Failed to parse TDS JSON")
        .map_err(|_| AppError::not_found(anyhow::anyhow!("Failed to parse TDS `{value}` JSON")))?;

    Ok(RestAPIResponse::success(tds))
}

pub async fn handle_del_tds() -> impl IntoResponse {}

pub async fn handle_put_ids() -> impl IntoResponse {}

pub async fn handle_get_ids() -> impl IntoResponse {}

pub async fn handle_del_ids() -> impl IntoResponse {}
