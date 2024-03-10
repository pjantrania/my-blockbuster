#[macro_use]
extern crate rocket;

use client::MyBlockbusterClient;
use rocket::{
    fairing::{self, AdHoc},
    fs::relative,
    fs::FileServer,
    Build, Rocket,
};
use rocket_db_pools::{sqlx, Database};
use rocket_dyn_templates::Template;

mod api;
mod client;
mod model;
mod omdb;
mod web;

#[derive(Database)]
#[database("sqlite_movies")]
struct Movies(sqlx::SqlitePool);

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    match Movies::fetch(&rocket) {
        Some(db) => match sqlx::migrate!("./migrations").run(&**db).await {
            Ok(_) => Ok(rocket),
            Err(e) => {
                error!("Failed to run database migrations: {}", e);
                Err(rocket)
            }
        },
        None => Err(rocket),
    }
}

#[launch]
fn rocket() -> _ {
    let migrations_fairing = AdHoc::try_on_ignite("SQLx Migrations", run_migrations);

    let http_client = reqwest::Client::new();
    let client = MyBlockbusterClient::new(http_client, String::from("http://localhost:8000"));

    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_ansi(false)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();
    rocket::build()
        .mount(
            "/",
            routes![
                web::delete_movie,
                web::index,
                web::movie_detail,
                web::new_movie_form,
                web::search_result_form,
                web::toggle_watched,
                web::add_movie,
            ],
        )
        .mount(
            "/api",
            routes![
                api::add_movie,
                api::delete_by_id,
                api::toggle_watched,
                api::omdb_search,
                api::get_movies,
                api::get_movie,
            ],
        )
        .mount("/public", FileServer::from(relative!("static")))
        .register("/", catchers![web::not_found])
        .attach(Template::fairing())
        .attach(Movies::init())
        .attach(migrations_fairing)
        .manage(client)
}
