#[macro_use]
extern crate rocket;
use rocket::{form::Strict, fs::relative, fs::FileServer};
use rocket_db_pools::{sqlx, Database};
use rocket_dyn_templates::Template;

mod api;
mod omdb;
mod root;

#[derive(Database)]
#[database("sqlite_movies")]
struct Movies(sqlx::SqlitePool);

#[derive(FromForm)]
struct MovieInput<'r> {
    title: Strict<&'r str>,
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![root::index, root::new_movie_form, root::search_result_form],
        )
        .mount("/api", routes![api::add_movie])
        .mount("/public", FileServer::from(relative!("static")))
        .attach(Template::fairing())
        .attach(Movies::init())
}
