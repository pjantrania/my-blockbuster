use crate::model::omdb::{OmdbMovie, OmdbResponse};
use crate::model::{MovieDetail, ResponseResult, SearchResponse};

use std::env;

pub async fn search_omdb(query: &str) -> ResponseResult<SearchResponse> {
    let uri = format!("http://www.omdbapi.com/?s={}", query);
    serde_json::from_str::<OmdbResponse>(get_and_parse_response(uri.as_str()).await.as_str())
        .unwrap()
        .into()
}

pub async fn get_movie(imdb_id: &str) -> MovieDetail {
    let uri = format!("http://www.omdbapi.com/?i={}", imdb_id);
    serde_json::from_str::<OmdbMovie>(get_and_parse_response(uri.as_str()).await.as_str())
        .unwrap()
        .into()
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
