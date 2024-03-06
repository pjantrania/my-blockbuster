use crate::client::MyBlockbusterClient;
use crate::model::ResponseResult;
use crate::omdb::{search_omdb, OmdbResponse, SearchResult};
use crate::Movies;
use rocket::form::Form;
use rocket::response::Redirect;
use rocket::{Request, State};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};
use serde::Serialize;
use tracing::{event, Level};

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
    let res = match sqlx::query_as::<_, Movie>("select * from movie limit 100")
        .fetch_all(&mut **db)
        .await
    {
        Ok(ms) => ms,
        Err(e) => {
            event!(Level::ERROR, "Could not fetch movies: {}", e);
            vec![]
        }
    };

    let add_uri = uri!(new_movie_form);
    Template::render("index", context! {items: res, add_uri: add_uri.to_string()})
}

#[get("/movie?<id>")]
pub async fn movie_detail(mut db: Connection<Movies>, id: i32) -> Option<Template> {
    match sqlx::query_as::<_, Movie>("select * from movie where id = ?")
        .bind(id)
        .fetch_one(&mut **db)
        .await
    {
        Ok(res) => Some(Template::render("movie", context! {m: res})),
        Err(e) => {
            tracing::error!("Error fetching movie with id = {}: {}", id, e);
            None
        }
    }
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

#[derive(FromForm)]
pub struct MovieIdInput {
    id: i32,
}

#[post("/delete", data = "<id_form>")]
pub async fn delete_movie(
    client: &State<MyBlockbusterClient>,
    id_form: Form<MovieIdInput>,
) -> Redirect {
    match client.delete_movie(id_form.id).await {
        ResponseResult::Response(res) => tracing::info!(
            "Successfully deleted movie with id {} and title {}.",
            res.movie_id,
            res.title
        ),
        ResponseResult::ErrorResponse(e) => tracing::error!("Error deleting movie: {}", e.err),
    };

    Redirect::to("/")
}

#[post("/toggleWatched", data = "<id_form>")]
pub async fn toggle_watched(
    client: &State<MyBlockbusterClient>,
    id_form: Form<MovieIdInput>,
) -> Redirect {
    match client.toggle_watched(id_form.id).await {
        ResponseResult::Response(res) => {
            tracing::info!(
                "Successfully switched watched to {} for movie with id {}.",
                res.watched,
                res.movie_id,
            )
        }
        ResponseResult::ErrorResponse(e) => {
            tracing::error!("Error toggling watched: {}", e.err)
        }
    };

    Redirect::to("/")
}

#[catch(404)]
pub fn not_found(req: &Request<'_>) -> Template {
    Template::render(
        "error/404",
        context! {
            uri: req.uri()
        },
    )
}
