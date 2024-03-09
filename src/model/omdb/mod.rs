use serde::Deserialize;

use super::{MovieDetail, MovieRating};

#[derive(Deserialize, sqlx::FromRow, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct OmdbSearchResult {
    pub title: String,
    pub year: String,
    #[serde(rename = "imdbID")]
    pub imdb_id: String,
    #[serde(rename = "Type")]
    pub result_type: String,
    #[serde(rename = "Poster")]
    pub poster_uri: String,
}

#[derive(Deserialize)]
pub struct OmdbSearchResponse {
    #[serde(rename = "Search")]
    pub results: Vec<OmdbSearchResult>,
    #[serde(rename = "totalResults")]
    pub total_results: String,
}

#[derive(Deserialize)]
pub struct OmdbErrorResponse {
    #[serde(rename = "Error")]
    pub error: String,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
#[serde(tag = "Response")]
pub enum OmdbResponse {
    #[serde(rename = "True")]
    Success(OmdbSearchResponse),
    #[serde(rename = "False")]
    Error(OmdbErrorResponse),
}

#[derive(Deserialize, sqlx::FromRow, Debug)]
#[serde(rename_all(deserialize = "PascalCase"))]
pub struct OmdbMovie {
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
    #[serde(skip_serializing)]
    pub ratings: Option<Vec<OmdbRating>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "PascalCase"))]
pub struct OmdbRating {
    pub source: String,
    pub value: String,
}
impl Into<MovieRating> for OmdbRating {
    fn into(self) -> MovieRating {
        MovieRating {
            source: self.source,
            value: self.value,
        }
    }
}

impl Into<MovieDetail> for OmdbMovie {
    fn into(self) -> MovieDetail {
        MovieDetail {
            title: self.title,
            year: self.year,
            imdb_id: self.imdb_id,
            result_type: self.result_type,
            poster_uri: self.poster_uri,
            released: self.released,
            runtime: self.runtime,
            genre: self.genre,
            director: self.director,
            writer: self.writer,
            actors: self.actors,
            plot: self.plot,
            language: self.language,
            country: self.country,
            awards: self.awards,
            metascore: self.metascore,
            imdb_rating: self.imdb_rating,
            imdb_votes: self.imdb_votes,
            dvd: self.dvd,
            box_office: self.box_office,
            production: self.production,
            website: self.website,
            ratings: self
                .ratings
                .map(|v| v.into_iter().map(|r| r.into()).collect()),
        }
    }
}
