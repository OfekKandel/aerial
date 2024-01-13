use super::http::ResponseError;
use reqwest::blocking::Response;

pub trait AuthClient {
    fn put_request(&self, endpoint: &str) -> Result<Response, ResponseError>;
}
