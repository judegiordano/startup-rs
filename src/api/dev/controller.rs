use actix_web::{
    web::{self, Path},
    HttpRequest, HttpResponse,
};
use anyhow::Result;
use chrono::Utc;
use serde::Deserialize;
use validator::Validate;

use crate::{
    models::{dev_data::DevLog, Model},
    util::{self, errors::ApiResponse},
};

pub async fn authenticate_dev(req: &HttpRequest) -> Result<bool> {
    let headers = req.headers();
    let token = match headers.get("Authorization") {
        Some(value) => value.to_str()?.split(' ').collect::<Vec<&str>>(),
        None => anyhow::bail!("unauthorized"),
    };
    let token = match token.get(1) {
        Some(value) => *value,
        None => anyhow::bail!("missing token"),
    };
    if token.ne("secret_lol") {
        anyhow::bail!("invalid token")
    }
    Ok(true)
}

#[derive(Debug, Validate, Deserialize)]
pub struct InsertBody {
    #[validate(length(
        min = 1,
        max = 20,
        message = "body.message must be between 1 and 20 characters"
    ))]
    message: String,
}

pub async fn ping(req: HttpRequest, body: web::Json<InsertBody>) -> ApiResponse {
    body.validate()?;
    authenticate_dev(&req).await?;
    let doc = DevLog {
        id: util::tools::nanoid(),
        message: body.message.to_owned(),
        date: Utc::now(),
    };
    DevLog::collection().await.insert_one(&doc, None).await?;
    Ok(HttpResponse::Ok().json(doc))
}

pub async fn pong(req: HttpRequest, id: Path<String>) -> ApiResponse {
    authenticate_dev(&req).await?;
    let doc = DevLog::read_by_id(&id).await?;
    Ok(HttpResponse::Ok().json(doc))
}
