use anyhow::Result;
use async_trait::async_trait;
use futures::stream::TryStreamExt;
use mongodb::{
    bson::{self, doc},
    results::CreateIndexesResult,
    Collection,
};
use serde::{de::DeserializeOwned, Serialize};

use crate::database::DATABASE;
pub mod dev_data;

pub async fn migrate() -> Result<()> {
    dev_data::DevLog::create_indexes().await?;
    Ok(())
}

#[async_trait]
pub trait Model<T: DeserializeOwned + Unpin + Send + Sync + Serialize> {
    fn collection_name() -> String;

    async fn create_indexes() -> Result<CreateIndexesResult>;

    async fn collection() -> Collection<T> {
        let name = Self::collection_name();
        DATABASE.get().await.collection::<T>(&name)
    }

    async fn read_by_id(_id: &str) -> Result<T> {
        let filter = doc! { "_id": _id };
        let result = Self::collection().await.find_one(filter, None).await?;
        match result {
            Some(doc) => Ok(doc),
            None => anyhow::bail!(format!("{} document not found", Self::collection_name())),
        }
    }

    async fn aggregate(pipeline: &[bson::Document]) -> Result<Vec<T>> {
        let pipeline = pipeline.to_owned();
        let mut results = Self::collection().await.aggregate(pipeline, None).await?;
        let mut docs = vec![];
        while let Some(doc) = results.try_next().await? {
            docs.push(bson::from_document(doc)?);
        }
        Ok(docs)
    }

    async fn count() -> Result<u64> {
        let count = Self::collection()
            .await
            .estimated_document_count(None)
            .await?;
        Ok(count)
    }
}
