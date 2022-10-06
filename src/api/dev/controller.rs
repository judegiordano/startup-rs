use anyhow::Result;
use axum::{extract::Path, Json};

use crate::{
    models::{dev_data::DevLog, Model},
    util,
};

pub async fn ping() -> Result<Json<DevLog>, util::errors::ApiError> {
    let doc = DevLog {
        id: util::tools::nanoid(),
        message: format!("request received: {}", chrono::Utc::now()),
    };
    DevLog::collection().await.insert_one(&doc, None).await?;
    Ok(Json(doc))
}

pub async fn pong(Path(id): Path<String>) -> Result<Json<DevLog>, util::errors::ApiError> {
    let doc = DevLog::read_by_id(&id).await?;
    Ok(Json(doc))
}
