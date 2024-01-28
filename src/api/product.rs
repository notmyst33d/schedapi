use std::sync::Arc;

use axum::response::Response;
use axum::body::Body;
use axum::extract::State;
use axum::routing::get;
use axum::Router;

use crate::data::SharedState;

async fn get_name(
    State(state): State<Arc<SharedState>>,
) -> &'static str {
    state.product_name
}

async fn get_logo_jpg(
    State(state): State<Arc<SharedState>>,
) -> Response {
    Response::builder()
        .header("Content-Type", "image/jpg")
        .body(Body::from(state.product_logo))
        .unwrap()
}

pub fn routes() -> Router<Arc<SharedState>> {
    Router::new()
        .route("/name", get(get_name))
        .route("/logo.jpg", get(get_logo_jpg))
}
