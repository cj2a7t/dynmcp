use axum::{
    routing::{get, put},
    Router,
};

use crate::{
    handler::{admin_handler, mcp_handler},
    model::app_state::AppState,
};

pub fn create_router(app_state: AppState) -> Router {
    Router::new()
        .route("/mcp/:instance_id", get(mcp_handler::handle_message))
        .route(
            "/admin/tds/:tds_id",
            get(admin_handler::handle_get_tds).delete(admin_handler::handle_del_tds),
        )
        .route(
            "/admin/ids/:ids_id",
            get(admin_handler::handle_get_ids).delete(admin_handler::handle_del_ids),
        )
        .route("/admin/tds", put(admin_handler::handle_put_tds))
        .route("/admin/ids", put(admin_handler::handle_put_ids))
        .with_state(app_state)
}
