use anyhow::Result;
use axum::{extract::Path, Json};
use serde::{Deserialize, Serialize};

use crate::{
    models::{dev_data::DevLog, Model},
    util,
};

#[derive(Deserialize, Serialize)]
pub struct Inserted {
    pub id: String,
}

pub async fn ping() -> Result<Json<Inserted>, util::errors::ApiError> {
    let doc = DevLog {
        id: util::tools::nanoid(),
        message: format!("request received: {}", chrono::Utc::now()),
    };
    let inserted = DevLog::collection().await.insert_one(doc, None).await?;
    Ok(Json(Inserted {
        id: inserted.inserted_id.to_string(),
    }))
}

pub async fn pong(Path(id): Path<String>) -> Result<Json<DevLog>, util::errors::ApiError> {
    let doc = DevLog::read_by_id(&id).await?;
    Ok(Json(doc))
}
