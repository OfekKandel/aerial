use super::spotify_api_handler::SpotifyApiHandler;
use super::spotify_api_spec::GotoNextTrack;
use super::spotify_api_spec::GotoPrevTrack;
use super::spotify_api_spec::Pause;
use super::spotify_api_spec::Resume;
use super::AuthError;
use super::MusicClient;
use crate::utils::api_handler::ApiHandler;
use crate::utils::api_spec::NoResponse;
use crate::utils::config::SpotifyConfig;
use crate::utils::http::ResponseError;
use crate::utils::Cache;
use thiserror::Error;

pub struct SpotifyClient {
    api_handler: SpotifyApiHandler,
}

#[derive(Error, Debug)]
pub enum SpotifyError {
    // #[error("There is no actively playing Spotify device")]
    // NoActiveDevice,
    #[error("Theres an error in the API request: {0}")]
    ApiRequestError(ResponseError),
}

impl MusicClient for SpotifyClient {
    type Error = SpotifyError;

    fn pause(&self) -> Result<NoResponse, SpotifyError> {
        self.api_handler
            .make_request(&Pause {})
            .map_err(SpotifyError::ApiRequestError)
    }

    fn resume(&self) -> Result<NoResponse, Self::Error> {
        self.api_handler
            .make_request(&Resume {})
            .map_err(SpotifyError::ApiRequestError)
    }

    fn goto_next_track(&self) -> Result<NoResponse, Self::Error> {
        self.api_handler
            .make_request(&GotoNextTrack {})
            .map_err(SpotifyError::ApiRequestError)
    }

    fn goto_prev_track(&self) -> Result<NoResponse, Self::Error> {
        self.api_handler
            .make_request(&GotoPrevTrack {})
            .map_err(SpotifyError::ApiRequestError)
    }
}

impl SpotifyClient {
    pub fn new(config: &SpotifyConfig, cache: &mut Cache) -> Result<Self, AuthError> {
        Ok(Self {
            api_handler: SpotifyApiHandler::new(config, cache)?,
        })
    }

    // fn get_playback_state(&self) {
    //     self.auth.get_request("me/player/prev")
    // }
}
