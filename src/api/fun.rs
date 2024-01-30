use std::sync::Arc;

use axum::body::Body;
use axum::extract::State;
use axum::response::Response;
use axum::routing::get;
use axum::Router;

use crate::data::SharedState;

async fn get_blaze(State(state): State<Arc<SharedState>>) -> Response {
    Response::builder()
        .header("Content-Type", "image/jpg")
        .body(Body::from(state.blaze))
        .unwrap()
}

pub fn routes() -> Router<Arc<SharedState>> {
    Router::new()
        .route("/blaze", get(get_blaze))
}
