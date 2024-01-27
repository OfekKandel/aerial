use std::collections::HashMap;
use reqwest::{blocking::RequestBuilder, header::HeaderMap, Method};
use serde::{de::DeserializeOwned, Deserialize};

pub trait ApiRequestSpec {
    type Resposne: DeserializeOwned;
    fn request(&self) -> ApiRequest;

    fn build(&self, api_endpoint: &str) -> RequestBuilder {
        let request = self.request();
        let endpoint = format!("{}/{}", api_endpoint, request.endpoint);
        let url = reqwest::Url::parse_with_params(endpoint.as_str(), request.params.unwrap_or_default()).unwrap();
        reqwest::blocking::Client::new()
            .request(request.method, url)
            .headers(request.headers.unwrap_or(HeaderMap::new()))
    }
}

pub struct ApiRequest {
    pub method: Method,
    pub endpoint: String,
    pub headers: Option<HeaderMap>,
    pub params: Option<HashMap<String, String>>,
}

impl ApiRequest {
    pub fn basic(method: Method, endpoint: &str) -> Self {
        Self::basic_with_form(method, endpoint, None)
    }

    pub fn basic_with_form(method: Method, endpoint: &str, form: Option<HashMap<String, String>>) -> Self {
        ApiRequest {
            method,
            endpoint: endpoint.into(),
            headers: None,
            params: form,
        }
    }
}

#[derive(Deserialize)]
pub struct NoResponse {}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum OptionalResponse<T> {
    Some(T),
    None(NoResponse)
}

impl<T> From<OptionalResponse<T>> for Option<T> {
    fn from(value: OptionalResponse<T>) -> Self {
        match value {
            OptionalResponse::Some(v) => Some(v),
            OptionalResponse::None(_) => None,
        }
    }
}

#[macro_export]
macro_rules! impl_endpoint {
    ($spec:ident, $method:path, $endpoint:expr, $response:ident) => {
        impl ApiRequestSpec for $spec {
            type Resposne = $response;

            fn request(&self) -> ApiRequest {
                ApiRequest::basic($method, $endpoint)
            }
        }
    };
}
