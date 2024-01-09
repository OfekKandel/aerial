use reqwest::StatusCode;
use thiserror::Error;

use super::spotify_auth::SpotifyAuthClient;
use super::MusicClient;
use crate::modules::music::spotify_auth::InitialAuthError;
use crate::utils::http::ResponseError;
use crate::utils::http::{ResponseError::InvalidResposne, ResponseValidationError::BadStatusCode};

pub struct SpotifyClient {
    auth: SpotifyAuthClient,
}

impl SpotifyClient {
    pub fn new() -> Result<Self, InitialAuthError> {
        Ok(Self {
            auth: SpotifyAuthClient::new("".into(), "".into())?,
        })
    }
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
}
