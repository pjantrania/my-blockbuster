use rocket::serde::{Deserialize, Serialize};

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
