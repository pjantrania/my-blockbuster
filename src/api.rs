use crate::{
    model::{
        AddMovieRequest, AddMovieResponse, DeleteResponse, ErrorResponse, ResponseResult,
        WatchedToggled,
    },
    omdb::{self, search_omdb, OmdbResponse},
    Movies,
};
use rocket::serde::json::Json;
use rocket_db_pools::{sqlx, Connection};

#[post("/movie", data = "<movie>")]
pub async fn add_movie(
    mut db: Connection<Movies>,
    movie: Json<AddMovieRequest>,
) -> Json<ResponseResult<AddMovieResponse>> {
    let existance: (i64,) = sqlx::query_as("select count(*) from movie where imdb_id = ?")
        .bind(&movie.imdb_id)
        .fetch_one(&mut **db)
        .await
        .unwrap();

    if existance.0 == 0 {
        let res = omdb::get_movie(movie.imdb_id.as_str()).await;
        let insert_stmt = res.get_insert_statement();
        let q =
            res.bind_insert_statement(sqlx::query_as::<_, AddMovieResponse>(insert_stmt.as_ref()));
        match q.fetch_one(&mut **db).await {
            Ok(res) => Json(ResponseResult::Response(res)),
            Err(err) => Json(ResponseResult::ErrorResponse(ErrorResponse {
                err: err.to_string(),
            })),
        }
    } else {
        Json(ResponseResult::ErrorResponse(ErrorResponse {
            err: format!("movie already exists: {}", movie.imdb_id),
        }))
    }
}

#[delete("/movie/<id>")]
pub async fn delete_by_id(
    mut db: Connection<Movies>,
    id: i32,
) -> Json<ResponseResult<DeleteResponse>> {
    match sqlx::query_as::<_, DeleteResponse>(
        "delete from movie where id = ? returning id as movie_id, title",
    )
    .bind(id)
    .fetch_one(&mut **db)
    .await
    {
        Ok(res) => Json(ResponseResult::Response(res)),
        Err(e) => {
            tracing::error!("Error deleting movie with id = {}: {}", id, e);
            Json(ResponseResult::ErrorResponse(ErrorResponse {
                err: e.to_string(),
            }))
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

#[get("/omdb/search/<query>")]
pub async fn omdb_search(query: &str) -> Json<OmdbResponse> {
    Json(search_omdb(query).await)
}
