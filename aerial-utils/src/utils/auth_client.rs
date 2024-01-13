use super::http::ResponseError;
use reqwest::blocking::Response;

pub trait AuthClient {
    fn post_request(&self, endpoint: &str) -> Result<Response, ResponseError>;
    fn put_request(&self, endpoint: &str) -> Result<Response, ResponseError>;
}
