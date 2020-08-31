use crate::types::component::*;
use anyhow::Result;
use appstream::Collection;
use bb8_postgres::{bb8::PooledConnection, PostgresConnectionManager};
use tokio_postgres::NoTls;

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

    pub async fn dump(
        &self,
        mut conn: PooledConnection<'_, PostgresConnectionManager<NoTls>>,
    ) -> Result<()> {
        for c in self.collection.components.iter() {
            create_component(&mut conn, &c.id.to_string(), c).await?;
        }
        Ok(())
    }
}
