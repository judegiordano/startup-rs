use actix_web::{web::Path, HttpRequest, HttpResponse};
use anyhow::Result;
use chrono::Utc;

use crate::{
    models::{dev_data::DevLog, Model},
    util::{self, errors::ApiResponse},
};

pub async fn authenticate_dev(req: &HttpRequest) -> Result<()> {
    let headers = req.headers();
    let auth = headers.get("Authorization");
    if auth.is_none() {
        anyhow::bail!("unauthorized")
    }
    let token = auth.unwrap().to_str()?.split(' ').collect::<Vec<&str>>();
    if token.len() <= 1 {
        anyhow::bail!("missing token")
    }
    let token = token[1];
    if token != "secret_lol" {
        anyhow::bail!("invalid token")
    }
    Ok(())
}

pub async fn ping(req: HttpRequest) -> ApiResponse {
    authenticate_dev(&req).await?;
    let doc = DevLog {
        id: util::tools::nanoid(),
        message: String::from("request received"),
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
