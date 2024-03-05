use crate::model::{DeleteResponse, ErrorResponse};

pub struct MyBlockbusterClient {
    client: reqwest::Client,
}

impl MyBlockbusterClient {
    pub fn new(client: reqwest::Client) -> Self {
        return MyBlockbusterClient { client };
    }

    pub async fn delete_movie(&self, id: i32) -> Result<DeleteResponse, ErrorResponse> {
        Ok(serde_json::from_str::<DeleteResponse>(
            self.client
                .delete(format!("{}/api/movie/{}", "http://localhost:8000", id))
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
