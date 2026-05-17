mod db;
mod llm;
mod models;
mod routes;
mod safety;

use anyhow::Context;
use db::{init_db, seed_db};
use llm::{build_gemma_provider, DynGemmaProvider};
use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};
use std::{env, net::SocketAddr, str::FromStr, sync::Arc};
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
    pub gemma: DynGemmaProvider,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let database_url =
        env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://gemma_herbalcare.db".to_string());
    let options = SqliteConnectOptions::from_str(&database_url)?
        .create_if_missing(true)
        .foreign_keys(true);
    let pool = SqlitePool::connect_with(options)
        .await
        .with_context(|| format!("failed to connect to {database_url}"))?;

    init_db(&pool).await?;
    seed_db(&pool).await?;

    let state = AppState {
        pool,
        gemma: Arc::from(build_gemma_provider()),
    };

    let app = routes::router(state).layer(
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any),
    );

    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr: SocketAddr = format!("0.0.0.0:{port}").parse()?;
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!("Gemma HerbalCare backend listening on http://{addr}");
    axum::serve(listener, app).await?;
    Ok(())
}
