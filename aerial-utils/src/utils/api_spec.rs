use reqwest::{blocking::RequestBuilder, header::HeaderMap, Method};
use serde::{de::DeserializeOwned, Deserialize};

pub trait ApiRequestSpec {
    type Resposne: DeserializeOwned;
    fn request(&self) -> ApiRequest;

    fn build(&self, api_endpoint: &str) -> RequestBuilder {
        let request = self.request();
        let endpoint = format!("{}/{}", api_endpoint, request.endpoint);
        reqwest::blocking::Client::new()
            .request(request.method, endpoint)
            .headers(request.headers.unwrap_or(HeaderMap::new()))
            .form(&request.form.unwrap_or(Vec::new()))
    }
}

pub struct ApiRequest {
    pub method: Method,
    pub endpoint: String,
    pub headers: Option<HeaderMap>,
    pub form: Option<Vec<(String, String)>>,
}

impl ApiRequest {
    pub fn basic(method: Method, endpoint: &str) -> Self {
        ApiRequest {
            method,
            endpoint: endpoint.into(),
            headers: None,
            form: None,
        }
    }
}

#[derive(Deserialize)]
pub struct NoResponse {}
