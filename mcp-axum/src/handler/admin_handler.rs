use crate::{
    error::api_error::RestAPIError,
    middleware::api_key_auth::ApiKey,
    model::{api_response::RestAPIResponse, app_state::AppState},
};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use mcp_common::xds::{ids::IDS, tds::TDS};
use mcp_plugin::datasource::datasource::DataSource;

pub async fn handle_put_tds(
    _api_key: ApiKey,
    State(state): State<AppState>,
    Json(tds): Json<TDS>,
) -> Result<impl IntoResponse, RestAPIError> {
    tds.validate().map_err(RestAPIError::bad_request)?;
    state
        .data_source
        .put(&tds.id, &tds)
        .await
        .map_err(RestAPIError::internal)?;
    Ok(RestAPIResponse::success(tds))
}

pub async fn handle_get_tds(
    _api_key: ApiKey,
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, RestAPIError> {
    let tds = state
        .data_source
        .get::<TDS>(&id)
        .await
        .map_err(RestAPIError::internal)?;
    Ok(RestAPIResponse::success(tds))
}

pub async fn handle_del_tds(
    _api_key: ApiKey,
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, RestAPIError> {
    let res = state
        .data_source
        .delete(&id)
        .await
        .map_err(RestAPIError::internal)?;
    Ok(RestAPIResponse::success(format!(
        "TDS `{id}` delete result: {res}"
    )))
}

pub async fn handle_put_ids(
    _api_key: ApiKey,
    State(state): State<AppState>,
    Json(ids): Json<IDS>,
) -> Result<impl IntoResponse, RestAPIError> {
    ids.validate().map_err(RestAPIError::bad_request)?;
    state
        .data_source
        .put(&ids.id, &ids)
        .await
        .map_err(RestAPIError::internal)?;
    Ok(RestAPIResponse::success(ids))
}

pub async fn handle_get_ids(
    _api_key: ApiKey,
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, RestAPIError> {
    let ids = state
        .data_source
        .get::<IDS>(&id)
        .await
        .map_err(RestAPIError::internal)?;
    Ok(RestAPIResponse::success(ids))
}

pub async fn handle_del_ids(
    _api_key: ApiKey,
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, RestAPIError> {
    let res = state
        .data_source
        .delete(&id)
        .await
        .map_err(RestAPIError::internal)?;
    Ok(RestAPIResponse::success(format!(
        "IDS `{id}` delete result: {res}"
    )))
}
