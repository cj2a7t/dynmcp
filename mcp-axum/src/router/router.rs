use axum::{
    routing::{get, post, put},
    Router,
};

use crate::{
    handler::{admin_handler, healthz_handler, mcp_handler},
    model::app_state::AppState,
};

pub fn create_router(app_state: AppState) -> Router {
    Router::new()
        // Data Plane
        // TODO Stream[able] Http
        .route("/mcp/{ids_id}", post(mcp_handler::mcp_post))
        .route("/mcp/{ids_id}", get(mcp_handler::mcp_get))
        // health check
        .route("/healthz", get(healthz_handler::healthz))
        // Control Plane
        .route(
            "/admin/tds/{tds_id}",
            get(admin_handler::handle_get_tds).delete(admin_handler::handle_del_tds),
        )
        .route(
            "/admin/ids/{ids_id}",
            get(admin_handler::handle_get_ids).delete(admin_handler::handle_del_ids),
        )
        .route("/admin/tds/{tds_id}", put(admin_handler::handle_put_tds))
        .route("/admin/ids/{ids_id}", put(admin_handler::handle_put_ids))
        .route("/admin/tds", get(admin_handler::handle_get_all_tds))
        .route("/admin/ids", get(admin_handler::handle_get_all_ids))
        .with_state(app_state)
}
