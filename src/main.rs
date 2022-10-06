#[macro_use]
extern crate lazy_static;
use anyhow::Result;
use axum::Router;
use std::net::SocketAddr;
use tracing_subscriber::FmtSubscriber;

pub mod api;
pub mod models;
pub mod util;

#[tokio::main]
async fn main() -> Result<()> {
    // tracing
    let subscriber = FmtSubscriber::builder()
        .with_max_level(util::config::CONFIG.log_level)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;
    // lazy connect to db
    tracing::info!(
        "connected to {:#?}",
        util::database::DATABASE.get().await.name()
    );
    // run migrations
    {
        models::migrate().await?
    }
    // app config
    let app = Router::new()
        .nest("/api", api::routes())
        .into_make_service();
    let addr = SocketAddr::from(([0, 0, 0, 0], util::config::CONFIG.port));
    tracing::info!("listening on {}", addr);
    // launch
    axum::Server::bind(&addr).serve(app).await?;
    Ok(())
}
