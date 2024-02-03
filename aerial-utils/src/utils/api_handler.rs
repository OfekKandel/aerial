use super::{http::ResponseError, ApiRequestSpec};
use serde::{de::DeserializeOwned, Serialize};

pub trait ApiHandler {
    fn make_request<B: Serialize, R: DeserializeOwned>(&self, spec: &dyn ApiRequestSpec<Body = B, Resposne = R>) -> Result<R, ResponseError>;
}
