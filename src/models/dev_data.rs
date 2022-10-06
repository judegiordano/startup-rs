use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use mongodb::{bson::doc, options::IndexOptions, results::CreateIndexesResult, IndexModel};
use serde::{Deserialize, Serialize};

use super::Model;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DevLog {
    #[serde(rename = "_id")]
    pub id: String,
    pub message: String,
    pub date: DateTime<Utc>,
}

#[async_trait]
impl Model<Self> for DevLog {
    fn collection_name() -> String {
        String::from("dev_logs")
    }

    async fn create_indexes() -> Result<CreateIndexesResult> {
        let message_index = IndexModel::builder()
            .keys(doc! { "message": 1 })
            .options(IndexOptions::builder().unique(false).build())
            .build();
        let indexes = vec![message_index];
        let created = Self::collection()
            .await
            .create_indexes(indexes, None)
            .await?;
        Ok(created)
    }
}
