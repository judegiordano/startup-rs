use actix_cors::Cors;
use actix_web::{middleware, web::scope, App, HttpServer};
use anyhow::Result;

use crate::{api, models, util};

pub async fn run() -> Result<()> {
    // lazy connect to db
    tracing::info!(
        "connected to {:#?}",
        super::database::DATABASE.get().await.name()
    );
    // run migrations
    models::migrate().await?;
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
