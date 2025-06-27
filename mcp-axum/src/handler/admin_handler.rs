use crate::{
    error::api_error::RestAPIError,
    model::api_response::RestAPIResponse,
    utils::etcd_helper::{etcd_delete, etcd_get, etcd_put},
};
use axum::{extract::Path, response::IntoResponse, Json};
use mcp_common::constants::constants::mcp_cache_consts::{ETCD_IDS_PREFIX, ETCD_TDS_PREFIX};
use mcp_core::model::xds::{ids::IDS, tds::TDS};

pub async fn handle_put_tds(Json(tds): Json<TDS>) -> Result<impl IntoResponse, RestAPIError> {
    tds.validate().map_err(RestAPIError::bad_request)?;
    etcd_put(ETCD_TDS_PREFIX, &tds.id, &tds).await?;
    Ok(RestAPIResponse::success(tds))
}

pub async fn handle_get_tds(Path(id): Path<String>) -> Result<impl IntoResponse, RestAPIError> {
    let tds = etcd_get::<TDS>(ETCD_TDS_PREFIX, &id).await?;
    Ok(RestAPIResponse::success(tds))
}

pub async fn handle_del_tds(Path(id): Path<String>) -> Result<impl IntoResponse, RestAPIError> {
    let res = etcd_delete(ETCD_TDS_PREFIX, &id).await?;
    Ok(RestAPIResponse::success(format!(
        "TDS `{id}` delete result: {res}"
    )))
}

pub async fn handle_put_ids(Json(ids): Json<IDS>) -> Result<impl IntoResponse, RestAPIError> {
    ids.validate().map_err(RestAPIError::bad_request)?;
    etcd_put(ETCD_IDS_PREFIX, &ids.id, &ids).await?;
    Ok(RestAPIResponse::success(ids))
}

pub async fn handle_get_ids(Path(id): Path<String>) -> Result<impl IntoResponse, RestAPIError> {
    let ids = etcd_get::<IDS>(ETCD_IDS_PREFIX, &id).await?;
    Ok(RestAPIResponse::success(ids))
}

pub async fn handle_del_ids(Path(id): Path<String>) -> Result<impl IntoResponse, RestAPIError> {
    let res = etcd_delete(ETCD_IDS_PREFIX, &id).await?;
    Ok(RestAPIResponse::success(format!(
        "IDS `{id}` delete result: {res}"
    )))
}
