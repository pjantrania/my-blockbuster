use crate::model::{DeleteResponse, ErrorResponse};

pub struct MyBlockbusterClient {
    client: reqwest::Client,
    base_url: String,
}

impl MyBlockbusterClient {
    pub fn new(client: reqwest::Client, base_url: String) -> Self {
        return MyBlockbusterClient { client, base_url };
    }

    pub async fn delete_movie(&self, id: i32) -> Result<DeleteResponse, ErrorResponse> {
        Ok(serde_json::from_str::<DeleteResponse>(
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
        .unwrap())
    }
}
