use super::{AuthError, SpotifyAuthClient};
use crate::utils::{
    api_handler::ApiHandler,
    auth_client::AddAuthExt,
    config::SpotifyConfig,
    http::{ExtractFromResposneExt, ResponseError, ValidateResponseExt},
    ApiRequestSpec, Cache,
};
use reqwest::header::{CONTENT_LENGTH, CONTENT_TYPE};
use serde::{de::DeserializeOwned, Serialize};

const API_ENDPOINT: &str = "https://api.spotify.com/v1";

pub struct SpotifyApiHandler {
    pub auth: SpotifyAuthClient,
}

impl SpotifyApiHandler {
    pub fn new(config: &SpotifyConfig, cache: &mut Cache) -> Result<Self, AuthError> {
        Ok(Self {
            auth: SpotifyAuthClient::new(cache, config.client_id.as_str(), config.client_secret.as_str())?,
        })
    }
}

impl ApiHandler for SpotifyApiHandler {
    fn make_request<B: Serialize, R: DeserializeOwned>(&self, spec: &dyn ApiRequestSpec<Body = B, Resposne = R>) -> Result<R, ResponseError> {
        let request = spec
            .build(API_ENDPOINT)
            .auth(&self.auth)
            .header(CONTENT_TYPE, "application/json")
            .header(CONTENT_LENGTH, 0);
        Ok(request.send().validate()?.extract()?)
    }
}
