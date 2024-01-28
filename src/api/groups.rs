use std::sync::Arc;

use axum::extract::{Json, Query, State};
use axum::routing::{get, post};
use axum::Router;
use uuid::Uuid;

use crate::data::{
    GenericAccessTokenRequest, Group, GroupCreateRequest, GroupDeleteRequest, GroupWithoutSchedule,
    SharedState, UserComposite,
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
        epoch: None,
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

async fn post_delete(
    State(state): State<Arc<SharedState>>,
    Json(request): Json<GroupDeleteRequest>,
) -> axum::response::Result<&'static str> {
    let _: UserComposite = query_one!(
        &state.session,
        &state.queries.get_user_composite,
        (request.access_token,),
        "Access token is invalid"
    );

    query!(
        &state.session,
        &state.queries.delete_group,
        (request.group_id,),
        "Cannot delete a group"
    );

    Ok("Successfully deleted a group")
}

async fn get_list(
    State(state): State<Arc<SharedState>>,
    Query(request): Query<GenericAccessTokenRequest>,
) -> axum::response::Result<Json<Vec<GroupWithoutSchedule>>> {
    let groups: Vec<GroupWithoutSchedule>;
    if state.single_user {
        groups = query_all!(
            &state.session,
            &state.queries.get_all_groups_without_schedule,
            (),
            "Cannot get all groups"
        );
    } else {
        if let None = request.access_token {
            return Err("Invalid access token".into());
        };
        let _user_composite: UserComposite = query_one!(
            &state.session,
            &state.queries.get_user_composite,
            (request.access_token.unwrap(),),
            "Invalid access token"
        );

        // TODO: Query groups by group_scope
        todo!();
    }

    Ok(Json(groups))
}

pub fn routes() -> Router<Arc<SharedState>> {
    Router::new()
        .route("/create", post(post_create))
        .route("/delete", post(post_delete))
        .route("/list", get(get_list))
}
