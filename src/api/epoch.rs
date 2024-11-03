use crate::bad_request;
use crate::data::*;
use crate::SharedState;
use axum::extract::{Json, Query, State};
use axum::response::ErrorResponse;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::routing::post;
use axum::Router;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize)]
struct GroupRequest {
    group_id: Option<i64>,
}

#[derive(Deserialize)]
struct UpdateRequest {
    group_id: Option<i64>,
    epoch: Option<i64>,
}

#[derive(Serialize, Deserialize)]
struct Epoch {
    epoch: i64,
}

async fn post_update(
    State(state): State<Arc<SharedState>>,
    Json(request): Json<UpdateRequest>,
) -> axum::response::Result<impl IntoResponse, ErrorResponse> {
    if let Some(group_id) = request.group_id {
        state.storage.update_epoch(group_id, request.epoch).await?;
    } else {
        if request.epoch.is_none() {
            return Err(bad_request("epoch_none_for_kv"));
        }
        state
            .storage
            .update_kv("epoch", &request.epoch.unwrap().to_string())
            .await?;
    }
    Ok(Json(ApiResponse::Ok(())))
}

async fn get_epoch(
    State(state): State<Arc<SharedState>>,
    Query(request): Query<GroupRequest>,
) -> axum::response::Result<impl IntoResponse, ErrorResponse> {
    let mut epoch = state.storage.get_kv("epoch").await?.parse().unwrap();
    if let Some(group_id) = request.group_id {
        if let Some(group_epoch) = state.storage.get_group(group_id).await?.epoch {
            epoch = group_epoch;
        }
    };

    Ok(Json(ApiResponse::Ok(Epoch { epoch })))
}

pub async fn routes(state: Arc<SharedState>) -> Router<Arc<SharedState>> {
    let mut authorized_routes = state.authorized_routes.write().await;
    authorized_routes.push("/epoch/update");
    Router::new()
        .route("/", get(get_epoch))
        .route("/update", post(post_update))
}
