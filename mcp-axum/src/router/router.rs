use axum::{ routing::get, Router };

use crate::{ handler::mcp_handler, model::app_state::{ AppState } };

pub fn create_router(app_state: AppState) -> Router {
    Router::new()
        .route("/mcp:instance_id", get(mcp_handler::handle_message))
        .with_state(app_state)
}
