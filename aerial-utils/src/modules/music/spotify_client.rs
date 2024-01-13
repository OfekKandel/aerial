use super::AuthError;
use super::MusicClient;
use super::SpotifyAuthClient;
use crate::utils::config::SpotifyConfig;
use crate::utils::http::ResponseError;
use crate::utils::http::{ResponseError::InvalidResposne, ResponseValidationError::BadStatusCode};
use crate::utils::{AuthClient, Cache};
use reqwest::StatusCode;
use thiserror::Error;

pub struct SpotifyClient {
    auth: SpotifyAuthClient,
}

#[derive(Error, Debug)]
pub enum SpotifyError {
    #[error("There is no actively playing Spotify device")]
    NoActiveDevice,
    #[error("Theres an error in the API request: {0}")]
    ApiRequestError(ResponseError),
}

impl MusicClient for SpotifyClient {
    type Error = SpotifyError;

    fn pause(&self) -> Result<(), SpotifyError> {
        match self.auth.put_request("me/player/pause") {
            Ok(_) => Ok(()),
            Err(InvalidResposne(BadStatusCode(StatusCode::NOT_FOUND, body)))
                if body.contains("NO_ACTIVE_DEVICE") =>
            {
                Err(SpotifyError::NoActiveDevice)
            }
            Err(err) => Err(SpotifyError::ApiRequestError(err)),
        }
    }

    fn resume(&self) -> Result<(), Self::Error> {
        match self.auth.put_request("me/player/play") {
            Ok(_) => Ok(()),
            Err(InvalidResposne(BadStatusCode(StatusCode::NOT_FOUND, body)))
                if body.contains("NO_ACTIVE_DEVICE") =>
            {
                Err(SpotifyError::NoActiveDevice)
            }
            Err(err) => Err(SpotifyError::ApiRequestError(err)),
        }
    }
}

impl SpotifyClient {
    pub fn new(config: &SpotifyConfig, cache: &mut Cache) -> Result<Self, AuthError> {
        Ok(Self {
            auth: SpotifyAuthClient::new(
                cache,
                config.client_id.as_str(),
                config.client_secret.as_str(),
            )?,
        })
    }
}
