#[macro_use]
extern crate lazy_static;
use anyhow::Result;
use tracing_subscriber::FmtSubscriber;

pub mod api;
pub mod database;
pub mod models;
pub mod server;
pub mod tests;
pub mod util;

#[tokio::main]
async fn main() -> Result<()> {
    // tracing
    let subscriber = FmtSubscriber::builder()
        .with_max_level(util::config::CONFIG.log_level)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;
    // launch
    server::run().await?;
    Ok(())
}
