use anyhow::Result;
use appstream::{Collection, Component};
use bson::Document;
use serde_json::{Map, Value};
use std::convert::TryFrom;

#[derive(Debug)]
pub struct Repository {
    pub collection: Collection,
}

impl Repository {
    pub fn with_collection(collection_path: &str) -> Result<Self> {
        Ok(Self {
            collection: Collection::from_gzipped(collection_path.into())?,
        })
    }

    pub fn dump(&self) -> Result<()> {
        let client = mongodb::sync::Client::with_uri_str("mongodb://localhost:27017/")?;
        let coll = client.database("flathub").collection("components");
        coll.drop(None)?;

        self.collection.components.iter().for_each(|c| {
            let value: Map<String, Value> =
                serde_json::from_str(&serde_json::to_string(&c).unwrap()).unwrap();
            let document = Document::try_from(value).unwrap();

            coll.insert_one(document, None).unwrap();
        });
        Ok(())
    }
}
