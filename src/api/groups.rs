use std::sync::Arc;

use axum::extract::Json;
use axum::extract::State;
use axum::routing::post;
use axum::Router;
use uuid::Uuid;

use crate::data::Group;
use crate::data::GroupCreateRequest;
use crate::data::SharedState;

async fn post_create(
    State(state): State<Arc<SharedState>>,
    Json(request): Json<GroupCreateRequest>,
) -> axum::response::Result<Json<Group>> {
    let group = Group {
        id: Uuid::new_v4(),
        name: request.name.clone(),
        schedule: vec![],
    };

    if let Err(error) = state
        .session
        .query(
            "INSERT INTO groups (id, name, schedule) VALUES (?, ?, ?)",
            &group,
        )
        .await
    {
        return Err(error.to_string().into());
    };

    if let Err(error) = state
        .session
        .query(
            "INSERT INTO groups_name_id_composite (name, id) VALUES (?, ?)",
            (group.name.clone(), group.id),
        )
        .await
    {
        return Err(error.to_string().into());
    };

    if let Err(error) = state
        .session
        .query(
            "UPDATE users SET group_scope = group_scope + ? WHERE access_token = ?;",
            (vec![group.id], request.access_token.clone()),
        )
        .await
    {
        return Err(error.to_string().into());
    };

    Ok(Json(group))
}

pub fn routes() -> Router<Arc<SharedState>> {
    Router::new().route("/create", post(post_create))
}
