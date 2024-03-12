use rocket::request::{FromRequest, Outcome, Request};

pub enum RequestSource {
    Htmx,
    Static,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for RequestSource {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match req.headers().get_one("HX-Request") {
            None => Outcome::Success(RequestSource::Static),
            Some(_) => Outcome::Success(RequestSource::Htmx),
        }
    }
}
