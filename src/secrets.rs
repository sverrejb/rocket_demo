use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

#[derive(Debug)]
pub struct ApiKey<'r>(&'r str);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey<'r> {
    type Error = &'static str;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match req.headers().get_one("x-api-key") {
            Some(key) if key == "valid_api_key" => Outcome::Success(ApiKey(key)),
            Some(_) => {
                let error = "Wrong API key";
                req.local_cache(|| error);
                Outcome::Failure((Status::BadRequest, error))
            }
            None => {
                let error = "No api_key";
                req.local_cache(|| error);
                Outcome::Failure((Status::BadRequest, error))
            }
        }
    }
}

#[get("/")]
pub fn sensitive(key: ApiKey<'_>) -> String {
    format!("Sensitive data retrived with {:?}", key)
}
