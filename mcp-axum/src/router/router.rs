use axum::{ routing::get, Router };

use crate::{ handler::{admin_handler, mcp_handler}, model::app_state::AppState };

pub fn create_router(app_state: AppState) -> Router {
    Router::new()
        .route("/mcp/:instance_id", get(mcp_handler::handle_message))
        .route("/admin/tds/:tds_id", get(admin_handler::handle_get_tds))
        .with_state(app_state)
}
