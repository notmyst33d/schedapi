use std::sync::Arc;

use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};

use crate::db::create_user;
use crate::{
    data::{
        SharedState, User, UserChangePasswordRequest, UserComposite, UserTokenRequest,
        UserTokenResponse,
    },
    query,
};
use crate::{query_one, query_one_checked};

async fn post_create(
    State(state): State<Arc<SharedState>>,
    Json(request): Json<UserTokenRequest>,
) -> axum::response::Result<Json<UserTokenResponse>> {
    if state.single_user {
        return Err("Cannot create new users on single user instance".into());
    };

    let result: Result<User, _> = query_one_checked!(
        state.session,
        &state.queries.get_group,
        (&request.username,)
    );
    if let Ok(user) = result {
        if user.username == request.username {
            return Err("User with this username already exists".into());
        };
    };

    let user = if let Ok(user) = create_user(
        &state.session,
        &state.queries,
        request.username,
        request.password,
    )
    .await
    {
        user
    } else {
        return Err("Cannot create a new user".into());
    };

    Ok(Json(UserTokenResponse {
        access_token: user.access_token.to_string(),
    }))
}

async fn post_login(
    State(state): State<Arc<SharedState>>,
    Json(request): Json<UserTokenRequest>,
) -> axum::response::Result<Json<UserTokenResponse>> {
    let user: User = query_one!(
        &state.session,
        &state.queries.get_user,
        (request.username,),
        "User doesnt exist"
    );

    let hash = if let Ok(hash) = PasswordHash::new(&user.password) {
        hash
    } else {
        return Err("Password hash is invalid".into());
    };

    if let Err(_) = Argon2::default().verify_password(request.password.as_bytes(), &hash) {
        return Err("Invalid password".into());
    };

    Ok(Json(UserTokenResponse {
        access_token: user.access_token.to_string(),
    }))
}

async fn post_change_password(
    State(state): State<Arc<SharedState>>,
    Json(request): Json<UserChangePasswordRequest>,
) -> axum::response::Result<&'static str> {
    let user_composite: UserComposite = query_one!(
        &state.session,
        &state.queries.get_user_composite,
        (request.access_token,),
        "Access token is invalid"
    );

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hashed = if let Ok(result) = argon2.hash_password(request.password.as_bytes(), &salt) {
        result.to_string()
    } else {
        return Err("Could not hash the password".into());
    };

    query!(
        &state.session,
        &state.queries.update_user_password,
        (hashed, user_composite.username),
        "Could not update the password"
    );

    Ok("Password changed successfully")
}

pub fn routes() -> Router<Arc<SharedState>> {
    Router::new()
        .route("/create", post(post_create))
        .route("/login", post(post_login))
        .route("/change_password", post(post_change_password))
}
