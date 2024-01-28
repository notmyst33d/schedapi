use std::sync::Arc;

use axum::extract::{Json, Query, State};
use axum::routing::{get, post};
use axum::Router;
use uuid::uuid;

use crate::data::{
    EpochRequest, EpochResponse, EpochUpdateRequest, Group, Internal, SharedState,
    UserComposite,
};
use crate::{query, query_one};

async fn post_update(
    State(state): State<Arc<SharedState>>,
    Json(request): Json<EpochUpdateRequest>,
) -> axum::response::Result<&'static str> {
    let _: UserComposite = query_one!(
        &state.session,
        &state.queries.get_user_composite,
        (request.access_token,),
        "Access token is invalid"
    );

    match request.group_id {
        Some(group_id) => {
            let _ = query!(
                &state.session,
                &state.queries.update_group_epoch,
                (request.epoch, group_id),
                "Cannot update group epoch"
            );
        }
        None => {
            let _ = query!(
                &state.session,
                &state.queries.update_internal_epoch,
                (request.epoch, uuid!("00000000-0000-0000-0000-000000000000")),
                "Cannot update global epoch"
            );
        }
    };

    Ok("Successfully updated epoch")
}

async fn get_epoch(
    State(state): State<Arc<SharedState>>,
    Query(request): Query<EpochRequest>,
) -> axum::response::Result<Json<EpochResponse>> {
    match request.group_id {
        Some(group_id) => {
            let epoch;
            let group: Group = query_one!(
                &state.session,
                &state.queries.get_group,
                (group_id,),
                "Cannot get group"
            );

            if let Some(group_epoch) = group.epoch {
                epoch = group_epoch;
            } else {
                let internal: Internal = query_one!(
                    &state.session,
                    &state.queries.get_internal,
                    (uuid!("00000000-0000-0000-0000-000000000000"),),
                    "Cannot get internal data"
                );
                epoch = internal.epoch;
            }

            Ok(Json(EpochResponse { epoch }))
        }
        None => {
            if !state.single_user {
                return Err("Global epoch is only available in single user mode".into());
            }

            let internal: Internal = query_one!(
                &state.session,
                &state.queries.get_internal,
                (uuid!("00000000-0000-0000-0000-000000000000"),),
                "Cannot get internal data"
            );
            Ok(Json(EpochResponse {
                epoch: internal.epoch,
            }))
        }
    }
}

pub fn routes() -> Router<Arc<SharedState>> {
    Router::new()
        .route("/", get(get_epoch))
        .route("/update", post(post_update))
}
