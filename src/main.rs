#![feature(proc_macro_hygiene, decl_macro)]

// mod app;
// mod types;
#[macro_use]
extern crate rocket;
mod appstream;
mod types;
use appstream::{Component, Manifest};
use rocket::response::content;

#[get("/<app_id>")]
fn index(app_id: String) -> content::Json<String> {
    let manifest_path = "/var/lib/flatpak/appstream/flathub/x86_64/active/appstream.xml";
    let manifest: Manifest = Manifest::from_path(manifest_path.into()).unwrap();
    let component = manifest
        .components
        .into_iter()
        .find(|c| c.id.0 == app_id)
        .unwrap();
    let j = serde_json::to_string(&component).unwrap();
    let c = j.clone();
    content::Json(c)
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
    // let manifest_path = "/var/lib/flatpak/appstream/flathub/x86_64/active/appstream.xml";
    //let manifest: Manifest = Manifest::from_path(manifest_path.into()).unwrap();

    // let fedora = "/var/lib/flatpak/appstream/fedora/x86_64/appstream.xml.gz";
    //let manifest: Manifest = Manifest::from_gzipped(fedora.into()).unwrap();
}
