#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
mod routes;
mod types;
use appstream::{AppId, Collection, Component};
use bson::Document;
use rocket::http::Method;
use rocket::response::content;
use serde_json::{Map, Value};
use std::convert::TryFrom;

use rocket_cors::{AllowedHeaders, AllowedOrigins};
#[get("/<app_id>")]
fn index(app_id: String) -> anyhow::Result<content::Json<String>> {
    let client = mongodb::sync::Client::with_uri_str("mongodb://localhost:27017/")?;
    let coll = client.database("flathub").collection("components");
    let doc: Document = coll
        .find_one(Some(bson::doc! {"id" : app_id}), None)?
        .unwrap();
    let j = serde_json::to_string(&doc).unwrap();
    Ok(content::Json(j))
}

fn main() -> anyhow::Result<()> {
    let allowed_origins = AllowedOrigins::some_exact(&["http://localhost:1234"]);

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()?;

    rocket::ignite()
        .mount("/", rocket_cors::catch_all_options_routes())
        .mount("/", routes![index])
        .attach(cors)
        .launch();
    /*
    let client = mongodb::sync::Client::with_uri_str("mongodb://localhost:27017/")?;
    let coll = client.database("flathub").collection("components");
    let manifest_path = "/var/lib/flatpak/appstream/flathub/x86_64/active/appstream.xml";
    let manifest: Collection = Collection::from_path(manifest_path.into()).unwrap();

    manifest.components.into_iter().for_each(|c| {
        let value: Map<String, Value> =
            serde_json::from_str(&serde_json::to_string(&c).unwrap()).unwrap();
        let document = Document::try_from(value).unwrap();

        coll.insert_one(document, None).unwrap();
    });
    */
    Ok(())
}
