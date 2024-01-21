use std::sync::Arc;

use axum::extract::{Json, Query, State};
use axum::routing::{get, post};
use axum::Router;
use uuid::Uuid;

use crate::data::{
    GenericAccessTokenRequest, Group, GroupCreateRequest, SharedState, UserComposite,
};
use crate::{query, query_all, query_one};

async fn post_create(
    State(state): State<Arc<SharedState>>,
    Json(request): Json<GroupCreateRequest>,
) -> axum::response::Result<Json<Group>> {
    let user_composite: UserComposite = query_one!(
        &state.session,
        &state.queries.get_user_composite,
        (request.access_token,),
        "Access token is invalid"
    );
    let group = Group {
        id: Uuid::new_v4(),
        name: request.name.clone(),
        schedule: None,
    };

    query!(
        &state.session,
        &state.queries.add_group,
        &group,
        "Cannot create a new group"
    );
    query!(
        &state.session,
        &state.queries.append_group_scope,
        (vec![group.id], &user_composite.username),
        "Cannot create a new group"
    );

    Ok(Json(group))
}

async fn get_list(
    State(state): State<Arc<SharedState>>,
    Query(request): Query<GenericAccessTokenRequest>,
) -> axum::response::Result<Json<Vec<Group>>> {
    let user_composite: UserComposite = query_one!(
        &state.session,
        &state.queries.get_user_composite,
        (request.access_token,),
        "Access token is invalid"
    );

    let mut groups: Vec<Group> = vec![];
    if state.single_user {
        groups = query_all!(
            &state.session,
            &state.queries.get_all_groups,
            (),
            "Cannot get all groups"
        );
    } else {
        todo!();
    }

    Ok(Json(groups))
}

pub fn routes() -> Router<Arc<SharedState>> {
    Router::new()
        .route("/create", post(post_create))
        .route("/list", get(get_list))
}
