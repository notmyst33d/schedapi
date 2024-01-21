mod api;
mod data;

use std::env;
use std::error::Error;
use std::sync::Arc;

use axum::Router;
use openssl::ssl::{SslContextBuilder, SslMethod, SslVerifyMode};
use scylla::SessionBuilder;
use tokio::fs;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::data::Config;
use crate::data::SharedState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + 'static>> {
    let config: Config = toml::from_str(
        &fs::read_to_string(env::var("CONFIG").unwrap_or("config.toml".into())).await?,
    )?;

    let mut session_builder = SessionBuilder::new()
        .known_node(format!("{}:{}", config.database.host, config.database.port));

    if let Some(ssl_cert) = config.database.ssl_cert {
        let mut context_builder = SslContextBuilder::new(SslMethod::tls())?;
        context_builder.set_ca_file(ssl_cert)?;
        context_builder.set_verify(SslVerifyMode::PEER);
        session_builder = session_builder.ssl_context(Some(context_builder.build()));
    }

    if let (Some(user), Some(password)) = (config.database.user, config.database.password) {
        session_builder = session_builder.user(user, password);
    }

    let session = session_builder.build().await?;

    session
        .use_keyspace(config.database.keyspace, false)
        .await?;

    let state = Arc::new(SharedState { session });
    let state_router: Router<Arc<SharedState>> = Router::new()
        .nest("/schedule", api::schedule::routes())
        .nest("/users", api::users::routes())
        .nest("/groups", api::groups::routes())
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", api::Docs::openapi()))
        .layer(CorsLayer::permissive());

    let router: Router<()> = state_router.with_state(state);

    let listener = TcpListener::bind(format!("0.0.0.0:{}", config.main.port)).await?;
    axum::serve(listener, router).await?;

    Ok(())
}
