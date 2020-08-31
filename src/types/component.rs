use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio_postgres::Client;

#[derive(Serialize, Deserialize)]
pub struct Component {
    pub id: i32,
    pub app_id: String,
    pub appstream: appstream::Component,
}

pub async fn create_component<'a>(
    conn: &mut Client,
    app_id: &'a str,
    component: &appstream::Component,
) -> Result<()> {
    conn.execute(
        "INSERT INTO components (app_id, appstream) VALUES ($1, $2)",
        &[&app_id, &serde_json::to_value(component)?],
    )
    .await?;
    Ok(())
}
