use std::sync::Arc;

use axum::extract::State;
use axum::routing::get;
use axum::Json;
use axum::Router;
use rand::Rng;

use crate::data::SharedState;
use crate::data::User;
use crate::data::UserCreateResponse;

const ACCESS_TOKEN_CHARACTERS: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";
const ACCESS_TOKEN_LENGTH: usize = 16;

async fn get_create(
    State(state): State<Arc<SharedState>>,
) -> axum::response::Result<Json<UserCreateResponse>> {
    let access_token: String = {
        let mut rng = rand::thread_rng();
        (0..ACCESS_TOKEN_LENGTH)
            .map(|_| {
                let idx = rng.gen_range(0..ACCESS_TOKEN_CHARACTERS.len());
                ACCESS_TOKEN_CHARACTERS[idx] as char
            })
            .collect()
    };

    let user = User {
        access_token: access_token.clone(),
        group_scope: vec![],
    };

    if let Err(error) = state
        .session
        .query(
            "INSERT INTO users (access_token, group_scope) VALUES (?, ?)",
            user,
        )
        .await
    {
        return Err(error.to_string().into());
    }

    Ok(Json(UserCreateResponse {
        access_token: access_token.clone(),
    }))
}

pub fn routes() -> Router<Arc<SharedState>> {
    Router::new().route("/create", get(get_create))
}
