use serde::{Deserialize, Serialize};
use std::env;

use crate::model::MovieDetail;

#[derive(Deserialize, Serialize, sqlx::FromRow, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SearchResult {
    pub title: String,
    pub year: String,
    #[serde(rename = "imdbID")]
    pub imdb_id: String,
    #[serde(rename = "Type")]
    pub result_type: String,
    #[serde(rename = "Poster")]
    pub poster_uri: String,
}

#[derive(Deserialize, Serialize)]
pub struct SearchResponse {
    #[serde(rename = "Search")]
    pub results: Option<Vec<SearchResult>>,
    #[serde(rename = "totalResults")]
    pub total_results: String,
}

#[derive(Deserialize, Serialize)]
pub struct ErrorResponse {
    #[serde(rename = "Error")]
    pub error: String,
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
#[serde(tag = "Response")]
pub enum OmdbResponse {
    #[serde(rename = "True")]
    Success(SearchResponse),
    #[serde(rename = "False")]
    Error(ErrorResponse),
}

pub async fn search_omdb(query: &str) -> OmdbResponse {
    let uri = format!("http://www.omdbapi.com/?s={}", query);
    serde_json::from_str::<OmdbResponse>(get_and_parse_response(uri.as_str()).await.as_str())
        .unwrap()
}

pub async fn get_movie(imdb_id: &str) -> MovieDetail {
    let uri = format!("http://www.omdbapi.com/?i={}", imdb_id);
    serde_json::from_str::<MovieDetail>(get_and_parse_response(uri.as_str()).await.as_str())
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
