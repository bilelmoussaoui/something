use appstream::Component;
use bb8_postgres::{bb8, PostgresConnectionManager};
use rocket::response::content;
use std::env;
use tokio_postgres::types::Json;

use std::str::FromStr;
use tokio_postgres::{Config, NoTls};

#[get("/apps")]
pub async fn all() -> content::Json<String> {
    let manager = PostgresConnectionManager::new(
        Config::from_str(&env::var("DATABASE_URL").expect("database url should be set")).unwrap(),
        NoTls,
    );
    let pool = bb8::Pool::builder()
        .max_size(15)
        .build(manager)
        .await
        .unwrap();

    let results: Vec<Json<Component>> = pool
        .get()
        .await
        .unwrap()
        .query("SELECT appstream FROM components WHERE app_id='org.gnome.design.Contrast'", &[])
        .await
        .unwrap()
        .iter()
        .map(|a| a.get(0))
        .collect::<Vec<Json<Component>>>();
    println!("{:#?}", results);
    content::Json("{'hey': 'world'}".to_string())
}
/*

#[get("/apps/<app_id>")]
pub fn index(app_id: String, conn: Database) -> content::Json<String> {
    let results: Vec<Json<Component>> = (&*conn).query("SELECT appstream FROM components", &[]).unwrap().iter().map(|a| a.get(0)).collect::<Vec<Json<Component>>>();
    let j = serde_json::to_string(&results).unwrap();
    content::Json(j)
}

#[get("/apps/category/<name>")]
pub fn category(name: String, conn: Database) -> content::Json<String> {
    let results: Vec<Json<Component>> = (&*conn).query("SELECT appstream FROM components", &[]).unwrap().iter().map(|a| a.get(0)).collect::<Vec<Json<Component>>>();
    let j = serde_json::to_string(&results).unwrap();
    content::Json(j)
}
*/
