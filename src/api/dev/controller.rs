use anyhow::Result;
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::util;

#[derive(Serialize, Deserialize)]
pub struct DevResponse {
    pub ok: bool,
    pub message: String,
}

fn try_me() -> Result<DevResponse> {
    Ok(DevResponse {
        ok: true,
        message: "sup".to_string(),
    })
}

pub async fn ping() -> Result<Json<DevResponse>, util::errors::ApiError> {
    let response = try_me()?;
    Ok(Json(response))
}
