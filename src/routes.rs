use crate::MyDatabase;
use anyhow::Result;
use appstream::enums::Category;
use bson::{doc, Document};
use rocket::response::content;
use rocket_contrib::databases::mongodb::cursor::Cursor;
use rocket_contrib::databases::mongodb::{self, db::ThreadedDatabase, ordered::OrderedDocument};
use std::iter::Iterator;
use std::str::FromStr;

#[get("/apps")]
pub fn all(conn: MyDatabase) -> Result<content::Json<String>> {
    let coll = (&*conn).collection("components");
    let cursor: Cursor = coll.find(Some(doc! {}), None)?;
    let docs: Vec<mongodb::error::Result<OrderedDocument>> = cursor.collect();
    let docs: Vec<&OrderedDocument> = docs
        .iter()
        .filter(|d| d.is_ok())
        .map(|d| d.as_ref().unwrap())
        .collect();

    let j = serde_json::to_string(&docs).unwrap();
    Ok(content::Json(j))
}

#[get("/apps/<app_id>")]
pub fn index(app_id: String, conn: MyDatabase) -> Result<content::Json<String>> {
    let coll = (&*conn).collection("components");
    let doc: OrderedDocument = coll.find_one(Some(doc! {"id" : app_id}), None)?.unwrap();
    let j = serde_json::to_string(&doc).unwrap();
    Ok(content::Json(j))
}

#[get("/apps/category/<name>")]
pub fn category(name: String, conn: MyDatabase) -> Result<content::Json<String>> {
    match Category::from_str(&name) {
        Ok(_) => {
            let coll = (&*conn).collection("components");

            let cursor: Cursor =
                coll.find(Some(doc! { "categories": { "$all" : [ name ] } }), None)?;

            let docs: Vec<mongodb::error::Result<OrderedDocument>> = cursor.collect();
            let docs: Vec<&OrderedDocument> = docs
                .iter()
                .filter(|d| d.is_ok())
                .map(|d| d.as_ref().unwrap())
                .collect();
            let j = serde_json::to_string(&docs).unwrap();
            Ok(content::Json(j))
        }
        _ => anyhow::bail!("invalid category"),
    }
}
