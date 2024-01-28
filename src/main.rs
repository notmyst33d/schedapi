mod api;
mod data;
mod db;
mod init;
mod serialization;

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

use crate::data::{Config, Queries, SharedState, User};
use crate::db::create_user;

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

    let queries = Queries::new(&session).await?;
    init::init(&session, &queries).await;

    if config.main.single_user {
        let result: Result<User, _> = query_one_checked!(session, &queries.get_user, ("admin",));
        if let Err(_) = result {
            println!("Creating admin user for single user mode");
            create_user(&session, &queries, "admin".to_string(), "admin".to_string()).await?;
        };
    };

    let product_logo = fs::read(config.product.logo).await?;
    let state = Arc::new(SharedState {
        session,
        queries,
        single_user: config.main.single_user,
        product_name: config.product.name.leak(),
        product_logo: product_logo.leak(),
    });
    let state_router: Router<Arc<SharedState>> = Router::new()
        .nest("/schedule", api::schedule::routes())
        .nest("/users", api::users::routes())
        .nest("/groups", api::groups::routes())
        .nest("/product", api::product::routes())
        .nest("/epoch", api::epoch::routes())
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", api::Docs::openapi()))
        .layer(CorsLayer::permissive());

    let router: Router<()> = state_router.with_state(state);

    let listener = TcpListener::bind(format!("0.0.0.0:{}", config.main.port)).await?;
    axum::serve(listener, router).await?;

    Ok(())
}
