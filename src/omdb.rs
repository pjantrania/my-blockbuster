use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct SearchResult {
    #[serde(rename(deserialize = "Title"))]
    pub title: String,
    #[serde(rename(deserialize = "Year"))]
    pub year: String,
    #[serde(rename(deserialize = "imdbID"))]
    pub imdb_id: String,
    #[serde(rename(deserialize = "Type"))]
    pub result_type: String,
    #[serde(rename(deserialize = "Poster"))]
    pub poster_uri: String,
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
    reqwest::get(format!(
        "http://www.omdbapi.com/?s={}&apiKey=",
        query
    ))
    .await
    .unwrap()
    .text()
    .await
    .map(|s| return serde_json::from_str::<OmdbResponse>(s.as_str()).unwrap())
    .unwrap()
}
