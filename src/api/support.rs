use crate::data::{SharedState, SupportContact};
use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};
use std::sync::Arc;

async fn support(State(state): State<Arc<SharedState>>) -> Json<Vec<SupportContact>> {
    Json(state.support.clone())
}

pub fn routes() -> Router<Arc<SharedState>> {
    Router::new().route("/", get(support))
}
