use crate::model::{
    AddMovieRequest, AddMovieResponse, DeleteResponse, GetMoviesResponse, ResponseResult,
    SearchResponse, WatchedToggled,
};

pub struct MyBlockbusterClient {
    client: reqwest::Client,
    base_url: String,
}

impl MyBlockbusterClient {
    pub fn new(client: reqwest::Client, base_url: String) -> Self {
        return MyBlockbusterClient { client, base_url };
    }

    pub async fn delete_movie(&self, id: i32) -> ResponseResult<DeleteResponse> {
        serde_json::from_str::<ResponseResult<DeleteResponse>>(
            self.client
                .delete(format!("{}/api/movie/{}", self.base_url, id))
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap()
                .as_str(),
        )
        .unwrap()
    }

    pub async fn toggle_watched(&self, id: i32) -> ResponseResult<WatchedToggled> {
        serde_json::from_str::<ResponseResult<WatchedToggled>>(
            self.client
                .put(format!("{}/api/movie/{}/watched", self.base_url, id))
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap()
                .as_str(),
        )
        .unwrap()
    }

    pub async fn add_movie(&self, imdb_id: &str) -> ResponseResult<AddMovieResponse> {
        serde_json::from_str::<ResponseResult<AddMovieResponse>>(
            self.client
                .post(format!("{}/api/movie", self.base_url))
                .json(&AddMovieRequest {
                    imdb_id: String::from(imdb_id),
                })
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap()
                .as_str(),
        )
        .unwrap()
    }

    pub async fn search_omdb(&self, query: &str) -> ResponseResult<SearchResponse> {
        serde_json::from_str::<ResponseResult<SearchResponse>>(
            self.client
                .get(format!("{}/api/omdb/search/{}", self.base_url, query))
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap()
                .as_str(),
        )
        .unwrap()
    }

    pub async fn get_movies(
        &self,
        count: Option<u32>,
        after: Option<u32>,
    ) -> ResponseResult<GetMoviesResponse> {
        serde_json::from_str::<ResponseResult<GetMoviesResponse>>(
            self.client
                .get(format!(
                    "{}/api/movies?count={}&after={}",
                    self.base_url,
                    count.unwrap_or(100),
                    after.unwrap_or(0),
                ))
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap()
                .as_str(),
        )
        .unwrap()
    }
}
