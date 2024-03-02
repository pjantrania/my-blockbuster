use crate::omdb::{search_omdb, OmdbResponse, SearchResult};
use crate::Movies;
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};
use serde::Serialize;

#[derive(sqlx::FromRow, Serialize, Debug)]
struct Movie {
    id: u32,
    #[sqlx(flatten)]
    movie: SearchResult,
    watched: bool,
    added: String,
}

#[get("/")]
pub async fn index(mut db: Connection<Movies>) -> Template {
    let res = sqlx::query_as::<_, Movie>("select * from movie limit 100")
        .fetch_all(&mut **db)
        .await
        .unwrap();

    println!("{:#?}", res[0]);
    let add_uri = uri!(new_movie_form);
    Template::render("index", context! {items: res, add_uri: add_uri.to_string()})
}

#[get("/add")]
pub fn new_movie_form() -> Template {
    Template::render("add", context! {})
}

#[get("/addSearchResults?<query>")]
pub async fn search_result_form(query: &str) -> Template {
    let res = search_omdb(query).await;
    match res {
        OmdbResponse::Success(res) => Template::render(
            "search_result",
            context! {items:res.results,add_uri:uri!(new_movie_form()).to_string(),query:query,},
        ),
        OmdbResponse::Error(e) => Template::render("add", context! {error:e.error,query:query}),
    }
}
