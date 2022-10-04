#[macro_use]
extern crate lazy_static;
use anyhow::Result;
use axum::{routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tracing_subscriber::FmtSubscriber;

pub mod util;

#[derive(Serialize, Deserialize)]
struct User {
    id: u64,
    username: String,
}

fn try_me() -> Result<User> {
    Ok(User {
        id: 1,
        username: String::from("jude"),
    })
}

async fn root() -> Result<Json<User>, util::errors::ApiError> {
    let user = try_me()?;
    Ok(Json(user))
}

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(util::config::CONFIG.log_level)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let app = Router::new().route("/", get(root));
    let addr = SocketAddr::from(([0, 0, 0, 0], util::config::CONFIG.port));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
