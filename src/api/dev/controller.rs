use actix_web::{get, post, web::Path, HttpResponse};
use anyhow::Result;

use crate::{
    models::{dev_data::DevLog, Model},
    util::{self, errors::ApiResponse},
};

pub async fn protect() -> Result<()> {
    Ok(())
}

#[post("/ping")]
pub async fn ping() -> ApiResponse {
    let doc = DevLog {
        id: util::tools::nanoid(),
        message: format!("request received: {}", chrono::Utc::now()),
    };
    let done = DevLog::collection().await.insert_one(&doc, None).await?;
    Ok(HttpResponse::Ok().json(done))
}

#[get("/pong/{id}")]
pub async fn pong(id: Path<String>) -> ApiResponse {
    let doc = DevLog::read_by_id(&id).await?;
    Ok(HttpResponse::Ok().json(doc))
}
