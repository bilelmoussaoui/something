#![feature(proc_macro_hygiene, decl_macro)]
use anyhow::Result;
#[macro_use]
extern crate rocket;
mod routes;
mod types;
use rocket::http::Method;
use rocket_cors::AllowedOrigins;

use rocket_contrib::database;
use rocket_contrib::databases::mongodb;

#[database("mongodb://localhost:27017/")]
pub struct MyDatabase(mongodb::db::Database);

fn main() -> Result<()> {
    let repo = types::Repository::with_collection(
        "/var/lib/flatpak/appstream/flathub/x86_64/active/appstream.xml.gz",
    )?;
    repo.dump()?;

    let allowed_origins = AllowedOrigins::some_exact(&["http://localhost:1234/"]);

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()?;
    rocket::ignite()
        .mount(
            "/api/v2/",
            routes![routes::index, routes::all, routes::category],
        )
        .attach(cors)
        .attach(MyDatabase::fairing())
        .launch();

    Ok(())
}
