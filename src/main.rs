#![feature(proc_macro_hygiene, decl_macro)]

// mod app;
// mod types;
#[macro_use]
extern crate rocket;
mod appstream;
mod types;
use appstream::{AppId, Collection, Component};
use rocket::response::content;

#[get("/<app_id>")]
fn index(app_id: String) -> content::Json<String> {
    let manifest_path = "/var/lib/flatpak/appstream/flathub/x86_64/active/appstream.xml";
    let manifest: Collection = Collection::from_path(manifest_path.into()).unwrap();
    let component = manifest
        .components
        .into_iter()
        .find(|c| c.id.0 == app_id)
        .unwrap();
    let j = serde_json::to_string(&component).unwrap();
    content::Json(j)
}

fn main() {
    //rocket::ignite().mount("/", routes![index]).launch();
    /*let c = Component::from_path("./src/appstream/tests/generic.xml".into()).unwrap();

        println!("{:#?}", c.releases);
    */
    let manifest_path = "/var/lib/flatpak/appstream/flathub/x86_64/active/appstream.xml";
    let collection: Collection = Collection::from_path(manifest_path.into()).unwrap();
    println!(
        "{:#?}",
        collection.find_by_id(AppId {
            0: "com.spotify.Client".to_string()
        })
    );
    // let fedora = "/var/lib/flatpak/appstream/fedora/x86_64/appstream.xml.gz";
    //let manifest: Collection = Collection::from_gzipped(fedora.into()).unwrap();
}
