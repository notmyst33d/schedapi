use crate::{ApiResponse, Group, SharedState};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{ErrorResponse, IntoResponse};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
struct GroupCreateRequest {
    name: String,
}

#[derive(Deserialize)]
struct IdRequest {
    id: i64,
}

async fn post_create(
    State(state): State<Arc<SharedState>>,
    Json(request): Json<GroupCreateRequest>,
) -> axum::response::Result<impl IntoResponse, ErrorResponse> {
    let result = state.storage.insert_group(&request.name).await?;
    Ok((
        StatusCode::OK,
        Json(ApiResponse::Ok(Group {
            id: result.last_insert_rowid(),
            epoch: None,
            name: request.name,
            schedule: None,
        })),
    ))
}

async fn post_delete(
    State(state): State<Arc<SharedState>>,
    Json(request): Json<IdRequest>,
) -> axum::response::Result<impl IntoResponse, ErrorResponse> {
    state.storage.delete_group(request.id).await?;
    Ok((StatusCode::OK, Json(ApiResponse::Ok(()))))
}

async fn get_list(
    State(state): State<Arc<SharedState>>,
) -> axum::response::Result<impl IntoResponse, ErrorResponse> {
    Ok((
        StatusCode::OK,
        Json(ApiResponse::Ok(state.storage.get_groups().await?)),
    ))
}

pub async fn routes(state: Arc<SharedState>) -> Router<Arc<SharedState>> {
    let mut authorized_routes = state.authorized_routes.write().await;
    authorized_routes.push("/groups/create");
    authorized_routes.push("/groups/delete");
    Router::new()
        .route("/create", post(post_create))
        .route("/delete", post(post_delete))
        .route("/list", get(get_list))
}
