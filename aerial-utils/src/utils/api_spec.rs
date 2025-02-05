use reqwest::{blocking::RequestBuilder, header::HeaderMap, Method};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::collections::HashMap;

pub trait ApiRequestSpec {
    type Resposne: DeserializeOwned;
    type Body: Serialize + Sized;

    // TODO: Change this to take self instead of a reference
    fn request(&self) -> ApiRequest<Self::Body>;

    fn build(&self, api_endpoint: &str) -> RequestBuilder {
        let request = self.request();
        let endpoint = format!("{}/{}", api_endpoint, request.endpoint);
        // TODO: Remove these unwraps
        let url = reqwest::Url::parse_with_params(endpoint.as_str(), request.params.unwrap_or_default()).unwrap();

        let mut req = reqwest::blocking::Client::new()
            .request(request.method, url)
            .headers(request.headers.unwrap_or(HeaderMap::new()));

        if let Some(body) = request.body {
            req = req.body(serde_json::to_string_pretty(&body).unwrap());
        }
        return req;
    }
}

pub struct ApiRequest<T: Serialize + Sized> {
    pub method: Method,
    pub endpoint: String,
    pub headers: Option<HeaderMap>,
    pub params: Option<HashMap<String, String>>,
    pub body: Option<T>,
}

impl<T: Serialize + Sized> ApiRequest<T> {
    pub fn basic(method: Method, endpoint: &str) -> Self {
        Self::basic_with_params(method, endpoint, None)
    }

    pub fn basic_with_params(method: Method, endpoint: &str, params: Option<HashMap<String, String>>) -> Self {
        ApiRequest {
            method,
            endpoint: endpoint.into(),
            headers: None,
            params,
            body: None,
        }
    }

    pub fn basic_with_body(method: Method, endpoint: &str, body: T) -> Self {
        ApiRequest {
            method,
            endpoint: endpoint.into(),
            headers: None,
            params: None,
            body: Some(body),
        }
    }
}

#[derive(Deserialize)]
pub struct NoResponse {}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum OptionalResponse<T> {
    Some(T),
    None(NoResponse),
}

impl<T> From<OptionalResponse<T>> for Option<T> {
    fn from(value: OptionalResponse<T>) -> Self {
        match value {
            OptionalResponse::Some(v) => Some(v),
            OptionalResponse::None(_) => None,
        }
    }
}

#[derive(Serialize)]
pub struct NoBody {}

#[macro_export]
macro_rules! impl_endpoint {
    // Implement a request without body or params
    ($spec:ident, $method:path, $endpoint:expr, $response:ident) => {
        impl ApiRequestSpec for $spec {
            type Resposne = $response;
            type Body = NoBody;

            fn request(&self) -> ApiRequest<Self::Body> {
                ApiRequest::basic($method, $endpoint)
            }
        }
    };
    // Implement a request with params
    ($spec:ident, $method:path, $endpoint:expr => $params_func:ident, $response:ident) => {
        impl ApiRequestSpec for $spec {
            type Resposne = $response;
            type Body = NoBody;

            fn request(&self) -> ApiRequest<Self::Body> {
                let params = $params_func(self);
                ApiRequest::basic_with_params($method, $endpoint, Some(params))
            }
        }
    };
    // Implement a request with a body
    ($spec:ident, $method:path, $endpoint:expr, $body_func:ident => $body:ident, $response:ident) => {
        impl ApiRequestSpec for $spec {
            type Resposne = $response;
            type Body = $body;

            fn request(&self) -> ApiRequest<Self::Body> {
                let body = $body_func(self);
                ApiRequest::basic_with_body($method, $endpoint, body)
            }
        }
    };
}
