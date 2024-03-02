use serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize, Serialize, sqlx::FromRow, Debug)]
#[serde(rename_all(deserialize = "PascalCase"))]
pub struct SearchResult {
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
    pub ratings: Option<Vec<OmdbRating>>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all(deserialize = "PascalCase"))]
pub struct OmdbRating {
    pub source: String,
    pub value: String,
}

#[derive(Deserialize, Serialize)]
pub struct SearchResponse {
    #[serde(rename(deserialize = "Search"))]
    pub results: Option<Vec<SearchResult>>,
    #[serde(rename(deserialize = "totalResults"))]
    pub total_results: String,
}

#[derive(Deserialize, Serialize)]
pub struct ErrorResponse {
    #[serde(rename(deserialize = "Error"))]
    pub error: String,
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "Response")]
pub enum OmdbResponse {
    #[serde(rename(deserialize = "True"))]
    Success(SearchResponse),
    #[serde(rename(deserialize = "False"))]
    Error(ErrorResponse),
}

pub async fn search_omdb(query: &str) -> OmdbResponse {
    let uri = format!("http://www.omdbapi.com/?s={}", query);
    serde_json::from_str::<OmdbResponse>(get_and_parse_response(uri.as_str()).await.as_str())
        .unwrap()
}

pub async fn get_movie(imdb_id: &str) -> SearchResult {
    let uri = format!("http://www.omdbapi.com/?i={}", imdb_id);
    serde_json::from_str::<SearchResult>(get_and_parse_response(uri.as_str()).await.as_str())
        .unwrap()
}

async fn get_and_parse_response(uri: &str) -> String {
    let omdb_key =
        env::var("OMDB_API_KEY").expect("Please provide OMDB_API_KEY as an environment variable.");

    reqwest::get(format!("{}&apiKey={}", uri, omdb_key))
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
}
