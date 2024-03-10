use rocket::serde::{Deserialize, Serialize};

use super::{Movie, OmdbErrorResponse, OmdbResponse, OmdbSearchResponse, SearchResult};

#[derive(Deserialize, Serialize, sqlx::FromRow)]
#[serde(crate = "rocket::serde")]
pub struct DeleteResponse {
    pub movie_id: i32,
    pub title: String,
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ErrorResponse {
    pub err: String,
}

#[derive(Deserialize, Serialize, sqlx::FromRow)]
#[serde(crate = "rocket::serde")]
pub struct WatchedToggled {
    pub movie_id: i32,
    pub watched: bool,
}

#[derive(Deserialize, Serialize, sqlx::FromRow)]
#[serde(crate = "rocket::serde")]
pub struct AddMovieResponse {
    pub movie_id: i32,
    pub title: String,
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde", tag = "type")]
pub enum ResponseResult<T> {
    Response(T),
    ErrorResponse(ErrorResponse),
}

impl From<OmdbErrorResponse> for ErrorResponse {
    fn from(item: OmdbErrorResponse) -> Self {
        ErrorResponse { err: item.error }
    }
}

#[derive(Deserialize, Serialize)]
pub struct SearchResponse {
    pub results: Vec<SearchResult>,
    pub total_results: u32,
}

impl From<OmdbSearchResponse> for SearchResponse {
    fn from(item: OmdbSearchResponse) -> Self {
        SearchResponse {
            results: item.results.into_iter().map(|i| i.into()).collect(),
            total_results: item.total_results.parse().unwrap(),
        }
    }
}

impl From<OmdbResponse> for ResponseResult<SearchResponse> {
    fn from(value: OmdbResponse) -> Self {
        match value {
            OmdbResponse::Success(res) => ResponseResult::Response(SearchResponse::from(res)),
            OmdbResponse::Error(err) => ResponseResult::ErrorResponse(ErrorResponse::from(err)),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct GetMoviesResponse {
    pub results: Vec<Movie>,
}
