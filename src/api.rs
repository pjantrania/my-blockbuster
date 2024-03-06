use crate::{
    model::{DeleteResponse, ErrorResponse, ResponseResult, WatchedToggled},
    omdb, root, MovieInput, Movies,
};
use rocket::{
    form::{Form, Strict},
    response::Redirect,
    serde::json::Json,
};
use rocket_db_pools::{sqlx, Connection};

#[post("/movie", data = "<movie>")]
pub async fn add_movie(mut db: Connection<Movies>, movie: Form<MovieInput<'_>>) -> Redirect {
    sqlx::query("insert into movie(title) values(?)")
        .bind(movie.title.to_string())
        .execute(&mut **db)
        .await
        .unwrap();
    Redirect::to("/")
}

#[delete("/movie/<id>")]
pub async fn delete_by_id(
    mut db: Connection<Movies>,
    id: i32,
) -> Result<Json<DeleteResponse>, Json<ErrorResponse>> {
    match sqlx::query_as::<_, DeleteResponse>(
        "delete from movie where id = ? returning id as movie_id, title",
    )
    .bind(id)
    .fetch_one(&mut **db)
    .await
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => {
            tracing::error!("Error deleting movie with id = {}: {}", id, e);
            Err(Json(ErrorResponse { err: e.to_string() }))
        }
    }
}

#[put("/movie/<id>/watched")]
pub async fn toggle_watched(
    mut db: Connection<Movies>,
    id: i32,
) -> Json<ResponseResult<WatchedToggled>> {
    match sqlx::query_as::<_, WatchedToggled>(
        "update movie set watched = not watched where id = ? returning id as movie_id, watched",
    )
    .bind(id)
    .fetch_one(&mut **db)
    .await
    {
        Ok(res) => Json(ResponseResult::Response(res)),
        Err(e) => Json(ResponseResult::ErrorResponse(ErrorResponse {
            err: e.to_string(),
        })),
    }
}

#[derive(FromForm)]
pub struct IdInput<'r> {
    imdb_id: Strict<&'r str>,
}

#[post("/movie/fromId", data = "<id_form>")]
pub async fn add_from_imdb_id(mut db: Connection<Movies>, id_form: Form<IdInput<'_>>) -> Redirect {
    let imdb_id = id_form.imdb_id.to_string();
    let existance: (i64,) = sqlx::query_as("select count(*) from movie where imdb_id = ?")
        .bind(&imdb_id)
        .fetch_one(&mut **db)
        .await
        .unwrap();
    if existance.0 == 0 {
        let result = omdb::get_movie(imdb_id.as_str()).await;

        sqlx::query(
            "insert into movie(
    title,
    year,
    imdb_id,
    result_type,
    poster_uri,
    released,
    runtime,
    genre,
    director,
    writer,
    actors,
    plot,
    language,
    country,
    awards,
    metascore,
    imdb_rating,
    imdb_votes,
    dvd,
    box_office,
    production,
    website
) values (
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?
)",
        )
        .bind(&result.title)
        .bind(&result.year)
        .bind(&result.imdb_id)
        .bind(&result.result_type)
        .bind(&result.poster_uri)
        .bind(&result.released)
        .bind(&result.runtime)
        .bind(&result.genre)
        .bind(&result.director)
        .bind(&result.writer)
        .bind(&result.actors)
        .bind(&result.plot)
        .bind(&result.language)
        .bind(&result.country)
        .bind(&result.awards)
        .bind(&result.metascore)
        .bind(&result.imdb_rating)
        .bind(&result.imdb_votes)
        .bind(&result.dvd)
        .bind(&result.box_office)
        .bind(&result.production)
        .bind(&result.website)
        .execute(&mut **db)
        .await
        .unwrap();
    }

    Redirect::to(uri!(root::index()))
}
