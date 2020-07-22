use anyhow::Result;
use appstream::Collection;

#[derive(Debug)]
pub struct Repository {
    pub collection: Collection,
}

impl Repository {
    pub fn with_collection(collection_path: &str) -> Result<Self> {
        Ok(Self {
            collection: Collection::from_path(collection_path.into())?,
        })
    }
}
