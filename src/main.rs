#[macro_use]
extern crate rocket;
use rocket::{
    fairing::{self, AdHoc},
    form::Strict,
    fs::relative,
    fs::FileServer,
    Build, Rocket,
};
use rocket_db_pools::{sqlx, Database};
use rocket_dyn_templates::Template;

mod api;
mod omdb;
mod root;

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

#[derive(FromForm)]
struct MovieInput<'r> {
    title: Strict<&'r str>,
}

#[launch]
fn rocket() -> _ {
    let migrations_fairing = AdHoc::try_on_ignite("SQLx Migrations", run_migrations);
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_ansi(false)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();
    rocket::build()
        .mount(
            "/",
            routes![
                root::index,
                root::new_movie_form,
                root::search_result_form,
                root::movie_detail,
            ],
        )
        .mount(
            "/api",
            routes![api::add_movie, api::add_from_imdb_id, api::delete_by_id],
        )
        .mount("/public", FileServer::from(relative!("static")))
        .register("/", catchers![root::not_found])
        .attach(Template::fairing())
        .attach(Movies::init())
        .attach(migrations_fairing)
}
