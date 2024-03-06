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

#[derive(Deserialize, Serialize, sqlx::FromRow)]
#[serde(crate = "rocket::serde")]
pub struct WatchedToggled {
    pub movie_id: i32,
    pub watched: bool,
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde", tag = "type")]
pub enum ResponseResult<T> {
    Response(T),
    ErrorResponse(ErrorResponse),
}
