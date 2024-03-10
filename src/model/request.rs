use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct AddMovieRequest {
    pub imdb_id: String,
}
