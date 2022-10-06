#[macro_use]
extern crate lazy_static;
use actix_cors::Cors;
use actix_web::{middleware, web::scope, App, HttpServer};
use anyhow::Result;
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
    tracing::info!("listening on port: {}", util::config::CONFIG.port);
    // launch
    HttpServer::new(move || {
        let cors = Cors::default();
        let compression = middleware::Compress::default();
        App::new()
            .wrap(cors)
            .wrap(compression)
            .service(scope("/api").configure(api::routes))
    })
    .bind(("0.0.0.0", util::config::CONFIG.port))?
    .run()
    .await?;
    Ok(())
}
