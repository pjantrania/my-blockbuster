use crate::client::MyBlockbusterClient;
use crate::model::ResponseResult;
use crate::web::request_source::RequestSource;
use rocket::form::Form;
use rocket::response::Redirect;
use rocket::{Request, State};
use rocket_dyn_templates::{context, Template};

use tracing::{event, Level};

#[get("/")]
pub async fn index() -> Template {
    Template::render("base", context! {})
}

#[get("/movies")]
pub async fn movies_list(
    client: &State<MyBlockbusterClient>,
    request_source: RequestSource,
) -> Template {
    let res = match client.get_movies(None, None).await {
        ResponseResult::Response(res) => res.results,
        ResponseResult::ErrorResponse(e) => {
            event!(Level::ERROR, "Could not fetch movies: {}", e.err);
            vec![]
        }
    };

    let add_uri = uri!(new_movie_form);
    match request_source {
        RequestSource::Htmx => Template::render(
            "snippets/index",
            context! {items: res, add_uri: add_uri.to_string()},
        ),
        RequestSource::Static => Template::render(
            "index.html",
            context! {items: res, add_uri: add_uri.to_string(), static_load: true},
        ),
    }
}

#[get("/movie?<id>")]
pub async fn movie_detail(
    client: &State<MyBlockbusterClient>,
    request_source: RequestSource,
    id: u32,
) -> Option<Template> {
    match client.get_movie(id).await {
        ResponseResult::Response(res) => Some(match request_source {
            RequestSource::Htmx => Template::render("snippets/movie", context! {m: res}),
            RequestSource::Static => {
                Template::render("movie.html", context! {m: res, static_load: true})
            }
        }),
        ResponseResult::ErrorResponse(e) => {
            tracing::error!("Error fetching movie with id = {}: {}", id, e.err);
            None
        }
    }
}

#[get("/add")]
pub fn new_movie_form(request_source: RequestSource) -> Template {
    match request_source {
        RequestSource::Htmx => Template::render("snippets/add", context! {}),
        RequestSource::Static => Template::render("add.html", context! {static_load: true}),
    }
}

#[derive(FromForm)]
pub struct ImdbIdInput {
    imdb_id: String,
}

#[post("/add", data = "<imdb_id_form>")]
pub async fn add_movie(
    client: &State<MyBlockbusterClient>,
    imdb_id_form: Form<ImdbIdInput>,
) -> Redirect {
    match client.add_movie(imdb_id_form.imdb_id.as_str()).await {
        ResponseResult::Response(res) => tracing::info!(
            "Successfully added movie with id {} and title {}.",
            res.movie_id,
            res.title
        ),
        ResponseResult::ErrorResponse(e) => tracing::error!("Error adding movie: {}", e.err),
    };
    Redirect::to("/")
}

#[get("/addSearchResults?<query>")]
pub async fn search_result_form(
    client: &State<MyBlockbusterClient>,
    request_source: RequestSource,
    query: &str,
) -> Template {
    let res = client.search_omdb(query).await;
    match request_source {
        RequestSource::Htmx => match res {
            ResponseResult::Response(res) => Template::render(
                "snippets/search_result",
                context! {items:res.results,add_uri:uri!(new_movie_form()).to_string(),query:query,},
            ),
            ResponseResult::ErrorResponse(e) => {
                Template::render("snippets/add", context! {error:e.err,query:query})
            }
        },
        RequestSource::Static => match res {
            ResponseResult::Response(res) => Template::render(
                "search_result.html",
                context! {items:res.results,add_uri:uri!(new_movie_form()).to_string(),query:query, static_load: true},
            ),
            ResponseResult::ErrorResponse(e) => Template::render(
                "add.html",
                context! {error:e.err,query:query, static_load: true},
            ),
        },
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
