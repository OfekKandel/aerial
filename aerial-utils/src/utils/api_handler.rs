use super::{http::ResponseError, ApiRequestSpec};
use serde::de::DeserializeOwned;

pub trait ApiHandler {
    fn make_request<T: DeserializeOwned>(
        &self,
        spec: &dyn ApiRequestSpec<Resposne = T>,
    ) -> Result<T, ResponseError>;
}
