use std::iter;

use itertools::Itertools;
use rocket::serde::{Deserialize, Serialize};
use sqlx::query::QueryAs;
use sqlx_sqlite::{Sqlite, SqliteArguments};

pub mod omdb;

use crate::model::omdb::*;
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

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct AddMovieRequest {
    pub imdb_id: String,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct Movie {
    pub id: u32,
    pub watched: bool,
    pub added: String,
    #[sqlx(flatten)]
    pub detail: MovieDetail,
}

#[derive(Deserialize, Serialize, sqlx::FromRow, Debug)]
pub struct MovieDetail {
    pub title: String,
    pub year: String,
    pub imdb_id: String,
    pub result_type: String,
    pub poster_uri: String,
    pub released: Option<String>,
    pub runtime: Option<String>,
    pub genre: Option<String>,
    pub director: Option<String>,
    pub writer: Option<String>,
    pub actors: Option<String>,
    pub plot: Option<String>,
    pub language: Option<String>,
    pub country: Option<String>,
    pub awards: Option<String>,
    pub metascore: Option<String>,
    pub imdb_rating: Option<String>,
    pub imdb_votes: Option<String>,
    pub dvd: Option<String>,
    pub box_office: Option<String>,
    pub production: Option<String>,
    pub website: Option<String>,
    #[sqlx(skip)]
    #[serde(skip_serializing)]
    pub ratings: Option<Vec<MovieRating>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MovieRating {
    pub source: String,
    pub value: String,
}

impl MovieDetail {
    pub fn get_insert_statement(&self) -> String {
        let as_map = serde_json::to_value(&self)
            .map(|v| v.as_object().unwrap().to_owned())
            .unwrap();

        let sorted_fields = as_map.keys().sorted().join(", ");
        let placeholders = iter::repeat("?").take(as_map.keys().len()).join(", ");

        format!(
            "insert into movie({}) values ({}) returning id as movie_id, title",
            sorted_fields, placeholders
        )
    }

    pub fn bind_insert_statement<'a>(
        &'a self,
        q: QueryAs<'a, Sqlite, AddMovieResponse, SqliteArguments<'a>>,
    ) -> QueryAs<'a, Sqlite, AddMovieResponse, SqliteArguments<'a>> {
        let as_map = serde_json::to_value(&self)
            .map(|v| v.as_object().unwrap().to_owned())
            .unwrap();
        let sorted_keys = as_map.keys().sorted();

        let mut r = q;
        for k in sorted_keys {
            let val = String::from(as_map[k].as_str().unwrap());
            r = r.bind(val);
        }
        r
    }
}

#[derive(Deserialize, Serialize)]
pub struct SearchResult {
    pub title: String,
    pub year: String,
    pub imdb_id: String,
    pub result_type: String,
    pub poster_uri: String,
}

impl Into<SearchResult> for OmdbSearchResult {
    fn into(self) -> SearchResult {
        SearchResult {
            title: self.title,
            year: self.year,
            imdb_id: self.imdb_id,
            result_type: self.result_type,
            poster_uri: self.poster_uri,
        }
    }
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
