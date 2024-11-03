mod api;
mod data;
mod storage;

use crate::data::*;
use crate::storage::Storage;
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use axum::http::{header, StatusCode};
use axum::response::IntoResponse;
use axum::Router;
use axum::{
    extract::{OriginalUri, Request, State},
    Json,
};
use axum::{
    middleware::{self, Next},
    response::ErrorResponse,
};
use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use sha2::Sha256;
use std::collections::BTreeMap;
use std::env;
use std::error::Error;
use std::sync::Arc;
use tokio::{fs, net::TcpListener, sync::RwLock};
use tower_http::cors::CorsLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

fn unauthorized() -> ErrorResponse {
    (
        StatusCode::UNAUTHORIZED,
        Json(ApiResponse::<()>::Err("unauthorized".into())),
    )
        .into()
}

fn bad_request(message: &'static str) -> ErrorResponse {
    (
        StatusCode::UNAUTHORIZED,
        Json(ApiResponse::<()>::Err(message.into())),
    )
        .into()
}

async fn auth_layer(
    State(state): State<Arc<SharedState>>,
    OriginalUri(uri): OriginalUri,
    request: Request,
    next: Next,
) -> axum::response::Result<impl IntoResponse, ErrorResponse> {
    if state
        .authorized_routes
        .read()
        .await
        .contains(&uri.path().to_string().as_str())
    {
        let cookie_header = request
            .headers()
            .get(header::COOKIE)
            .and_then(|h| h.to_str().ok())
            .ok_or(())
            .map_err(|_| unauthorized())?;
        let token = cookie_header
            .split("; ")
            .map(|s| s.split("=").collect::<Vec<&str>>())
            .filter(|s| s.get(0).and_then(|e| Some(*e == "token")).unwrap_or(false))
            .flat_map(|c| c.get(1).and_then(|v| Some(v.to_string())))
            .next()
            .ok_or(())
            .map_err(|_| unauthorized())?;
        let key: Hmac<Sha256> = Hmac::new_from_slice(state.jwt_secret.as_bytes()).unwrap();
        let claims: BTreeMap<String, String> =
            token.verify_with_key(&key).map_err(|_| unauthorized())?;
        let password = state.password.read().await;
        if !claims.get("sub").is_some_and(|s| s == "schedapiadmin") {
            return Err(unauthorized().into());
        }
        if !claims.get("psw").is_some_and(|s| *s == *password) {
            return Err(unauthorized().into());
        }
    }
    let response = next.run(request).await;
    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + 'static>> {
    let config: Config = toml::from_str(
        &fs::read_to_string(env::var("CONFIG").unwrap_or("config.toml".into())).await?,
    )?;

    let storage = Storage::open("data.db").await?;

    if let Err(_) = storage.get_kv("password").await {
        let hashed = Argon2::default()
            .hash_password(
                "defaultpassword".as_bytes(),
                &SaltString::generate(&mut OsRng),
            )
            .unwrap()
            .to_string();
        storage.insert_kv("password", &hashed).await?;
    }
    let password = storage.get_kv("password").await?;

    if let Err(_) = storage.get_kv("epoch").await {
        storage.insert_kv("epoch", "0".into()).await?;
    }

    let state = Arc::new(SharedState {
        storage,
        jwt_secret: config.jwt_secret,
        authorized_routes: RwLock::new(vec![]),
        password: RwLock::new(password),
        product_logo: fs::read(config.product.logo).await?.leak(),
        product_name: config.product.name.leak(),
    });
    let state_router = Router::new()
        .nest("/schedule", api::schedule::routes(state.clone()).await)
        .nest("/users", api::users::routes(state.clone()).await)
        .nest("/groups", api::groups::routes(state.clone()).await)
        .nest("/product", api::product::routes())
        .nest("/epoch", api::epoch::routes(state.clone()).await)
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", api::Docs::openapi()))
        .layer(CorsLayer::permissive())
        .layer(middleware::from_fn_with_state(state.clone(), auth_layer));

    let router: Router<()> = state_router.with_state(state);
    let listener = TcpListener::bind(format!("0.0.0.0:{}", config.port)).await?;
    axum::serve(listener, router).await?;

    Ok(())
}
