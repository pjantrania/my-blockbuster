use rocket::{
    form::Form,
    response::Redirect,
};
use rocket_db_pools::{sqlx, Connection};
use crate::{Movies, MovieInput};

#[post("/movie", data = "<movie>")]
pub async fn add_movie(mut db: Connection<Movies>, movie: Form<MovieInput<'_>>) -> Redirect {
    sqlx::query("insert into movie(title) values(?)")
        .bind(movie.title.to_string())
        .execute(&mut **db)
        .await.unwrap();
    Redirect::to("/")
}
