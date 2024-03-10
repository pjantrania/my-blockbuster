use crate::{
    model::{
        AddMovieRequest, AddMovieResponse, DeleteResponse, ErrorResponse, GetMoviesResponse, Movie,
        ResponseResult, SearchResponse, WatchedToggled,
    },
    omdb::{self, search_omdb},
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
pub async fn omdb_search(query: &str) -> Json<ResponseResult<SearchResponse>> {
    Json(search_omdb(query).await)
}

#[get("/movies?<count>&<after>")]
pub async fn get_movies(
    mut db: Connection<Movies>,
    count: Option<u32>,
    after: Option<u32>,
) -> Json<ResponseResult<GetMoviesResponse>> {
    let count = std::cmp::min(count.unwrap_or(100), 100);
    let after = after.unwrap_or(0);
    match sqlx::query_as::<_, Movie>("select * from movie order by id limit ? offset ?")
        .bind(count)
        .bind(after)
        .fetch_all(&mut **db)
        .await
    {
        Ok(ms) => Json(ResponseResult::Response(GetMoviesResponse { results: ms })),
        Err(e) => Json(ResponseResult::ErrorResponse(ErrorResponse {
            err: e.to_string(),
        })),
    }
}

#[get("/movie/<id>")]
pub async fn get_movie(mut db: Connection<Movies>, id: u32) -> Json<ResponseResult<Movie>> {
    match sqlx::query_as::<_, Movie>("select * from movie where id = ?")
        .bind(id)
        .fetch_optional(&mut **db)
        .await
    {
        Ok(res) => Json(res.map(|m| ResponseResult::Response(m)).unwrap_or(
            ResponseResult::ErrorResponse(ErrorResponse {
                err: String::from(format!("Couldn't find movie with id = {}", id)),
            }),
        )),
        Err(e) => Json(ResponseResult::ErrorResponse(ErrorResponse {
            err: e.to_string(),
        })),
    }
}
