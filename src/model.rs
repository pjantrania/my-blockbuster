use std::iter;

use itertools::Itertools;
use rocket::serde::{Deserialize, Serialize};
use sqlx::query::QueryAs;
use sqlx_sqlite::{Sqlite, SqliteArguments};

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

#[derive(sqlx::FromRow, Serialize, Debug)]
pub struct Movie {
    pub id: u32,
    pub watched: bool,
    pub added: String,
    #[sqlx(flatten)]
    pub detail: MovieDetail,
}

#[derive(Deserialize, Serialize, sqlx::FromRow, Debug)]
#[serde(rename_all(deserialize = "PascalCase"))]
pub struct MovieDetail {
    pub title: String,
    pub year: String,
    #[serde(rename(deserialize = "imdbID"))]
    pub imdb_id: String,
    #[serde(rename(deserialize = "Type"))]
    pub result_type: String,
    #[serde(rename(deserialize = "Poster"))]
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
    #[serde(rename(deserialize = "imdbRating"))]
    pub imdb_rating: Option<String>,
    #[serde(rename(deserialize = "imdbVotes"))]
    pub imdb_votes: Option<String>,
    #[serde(rename(deserialize = "DVD"))]
    pub dvd: Option<String>,
    pub box_office: Option<String>,
    pub production: Option<String>,
    pub website: Option<String>,
    #[sqlx(skip)]
    #[serde(skip_serializing)]
    pub ratings: Option<Vec<OmdbRating>>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all(deserialize = "PascalCase"))]
pub struct OmdbRating {
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
