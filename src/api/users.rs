use crate::{unauthorized, ApiResponse, SharedState};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use axum::extract::State;
use axum::http::{header, StatusCode};
use axum::response::{ErrorResponse, IntoResponse};
use axum::routing::post;
use axum::{Json, Router};
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use serde::Deserialize;
use sha2::Sha256;
use std::collections::BTreeMap;
use std::sync::Arc;

#[derive(Deserialize)]
struct PasswordRequest {
    password: String,
}

async fn post_login(
    State(state): State<Arc<SharedState>>,
    Json(request): Json<PasswordRequest>,
) -> axum::response::Result<impl IntoResponse, ErrorResponse> {
    let password = state.password.read().await;
    let hash = PasswordHash::new(&password).unwrap();
    if let Err(_) = Argon2::default().verify_password(request.password.as_bytes(), &hash) {
        return Err(unauthorized());
    }
    let key: Hmac<Sha256> = Hmac::new_from_slice(state.jwt_secret.as_bytes()).unwrap();
    let mut claims = BTreeMap::new();
    claims.insert("sub", "schedapiadmin");
    claims.insert("psw", &password);
    let token_str = claims.sign_with_key(&key).unwrap();
    Ok((
        StatusCode::OK,
        [(header::SET_COOKIE, format!("token={}", token_str))],
        Json(ApiResponse::Ok(())),
    ))
}

async fn post_change_password(
    State(state): State<Arc<SharedState>>,
    Json(request): Json<PasswordRequest>,
) -> axum::response::Result<impl IntoResponse, ErrorResponse> {
    let hashed = Argon2::default()
        .hash_password(
            request.password.as_bytes(),
            &SaltString::generate(&mut OsRng),
        )
        .unwrap()
        .to_string();
    state.storage.update_kv("password", &hashed).await?;
    *state.password.write().await = hashed;
    Ok((StatusCode::OK, Json(ApiResponse::Ok(()))))
}

pub async fn routes(state: Arc<SharedState>) -> Router<Arc<SharedState>> {
    let mut authorized_routes = state.authorized_routes.write().await;
    authorized_routes.push("/users/change_password");
    Router::new()
        .route("/login", post(post_login))
        .route("/change_password", post(post_change_password))
}
