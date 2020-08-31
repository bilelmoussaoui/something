#![feature(async_closure)]
#[macro_use]
extern crate log;
#[macro_use]
extern crate rocket;

mod routes;
mod types;
use bb8_postgres::{bb8, PostgresConnectionManager};
use dotenv::dotenv;
use rocket::Rocket;
use std::env;
use std::str::FromStr;
use tokio_postgres::{Config, NoTls};
mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("migrations");
}

#[launch]
async fn rocket() -> Rocket {
    dotenv().ok();

    let manager = PostgresConnectionManager::new(
        Config::from_str(&env::var("DATABASE_URL").expect("database url should be set")).unwrap(),
        NoTls,
    );
    let pool = bb8::Pool::builder()
        .max_size(15)
        .build(manager)
        .await
        .unwrap();
    /*
    match embedded::migrations::runner()
        .run_async(&mut pool.get().await.unwrap())
        .await
    {
        Ok(_) => {
            let repo = types::Repository::with_collection(
                "/var/lib/flatpak/appstream/flathub/x86_64/active/appstream.xml.gz",
            )
            .expect("failed to parse appstream file");
            repo.dump(pool.get().await.unwrap()).await;
        }
        Err(e) => {
            error!("Failed to run database migrations: {:?}", e);
        }
    }
    */

    /*
    let allowed_origins = AllowedOrigins::some_exact(&["http://localhost:1234/"]);

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()?;
    */
    rocket::ignite()
        //.attach(cors)
        .mount(
            "/api/v2/",
            routes![routes::all], // routes::index,  routes::category
        )
}
