mod api;
mod models;

use std::env;
use std::sync::Arc;

use axum::Router;
use tokio::fs;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::models::Config;
use crate::models::PortableScheduleEntry;
use crate::models::SharedState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let config: Config = toml::from_str(
        &fs::read_to_string(env::var("CONFIG").unwrap_or("config.toml".into())).await?,
    )?;

    let mut reader = csv::Reader::from_path(config.schedule)?;
    let data = reader
        .deserialize()
        .map(|e| e.unwrap())
        .collect::<Vec<PortableScheduleEntry>>();

    let state = Arc::new(SharedState { data });
    let state_router: Router<Arc<SharedState>> = Router::new()
        .nest("/schedule", api::schedule::routes())
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", api::Docs::openapi()))
        .layer(CorsLayer::permissive());

    let router: Router<()> = state_router.with_state(state);

    let listener = TcpListener::bind(format!("0.0.0.0:{}", config.port)).await?;

    axum::serve(listener, router).await?;
    Ok(())
}
