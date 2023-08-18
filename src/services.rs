use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{routes, AppState};

pub fn backend(app_state: Arc<AppState>) -> Router {
    // could add tower::ServiceBuilder here to group layers, especially if you add more layers.
    // see https://docs.rs/axum/latest/axum/middleware/index.html#ordering
    Router::new().merge(back_public_route(app_state.clone()))
}

pub fn back_public_route(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/contagem-pessoas", get(routes::people::count))
        .route("/pessoas", post(routes::people::post))
        .route("/pessoas", get(routes::people::get_by_search_term))
        .route("/pessoas/:id", get(routes::people::get_by_uuid))
        .with_state(app_state)
}
