use super::spotify_api_handler::SpotifyApiHandler;
use super::spotify_api_spec::{GetCurrentTrack, GetPlaybackState, GotoNextTrack, GotoPrevTrack, Pause, PlayingState, Resume};
use super::{AuthError, InitialAuthError, MusicClient};
use crate::utils::{api_handler::ApiHandler, config::SpotifyConfig, http::ResponseError, Cache};
use thiserror::Error;

pub struct SpotifyClient {
    pub api_handler: SpotifyApiHandler,
}

#[derive(Error, Debug)]
pub enum SpotifyError {
    #[error("There is no actively playing Spotify device")]
    NoActiveDevice,
    #[error("Action can't be performed when music is {0}")]
    UnwantedPlayingState(PlayingState),
    #[error("Theres an error in the API request: {0}")]
    ApiRequestError(ResponseError),
    #[error("Failed to initial authentication: {0}")]
    FailedInitialAuth(InitialAuthError),
}

impl MusicClient for SpotifyClient {
    type Error = SpotifyError;

    fn toggle(&self) -> Result<(), Self::Error> {
        match self.get_playing_state()? {
            Some(PlayingState::Playing) => self.pause(),
            _ => self.resume(),
        }
    }

    fn pause(&self) -> Result<(), SpotifyError> {
        self.verify_playing_state(PlayingState::Playing)?;
        self.api_handler.make_request(&Pause).map_err(SpotifyError::ApiRequestError)?;
        Ok(())
    }

    fn resume(&self) -> Result<(), Self::Error> {
        self.verify_playing_state(PlayingState::Paused)?;
        self.api_handler.make_request(&Resume).map_err(SpotifyError::ApiRequestError)?;
        Ok(())
    }

    fn goto_next_track(&self) -> Result<(), Self::Error> {
        self.verify_active_device()?;
        self.api_handler.make_request(&GotoNextTrack).map_err(SpotifyError::ApiRequestError)?;
        Ok(())
    }

    fn goto_prev_track(&self) -> Result<(), Self::Error> {
        self.verify_active_device()?;
        self.api_handler.make_request(&GotoPrevTrack).map_err(SpotifyError::ApiRequestError)?;
        Ok(())
    }

    fn print_curr_track(&self) -> Result<(), Self::Error> {
        self.verify_active_device()?;
        let curr_track = self.api_handler.make_request(&GetCurrentTrack).map_err(SpotifyError::ApiRequestError)?;
        match curr_track.item {
            Some(track) => Ok(println!("{}", track)),
            None => Ok(println!("There's no track currently playing")),
        }
    }
}

impl SpotifyClient {
    pub fn new(config: &SpotifyConfig, cache: &mut Cache) -> Result<Self, AuthError> {
        Ok(Self {
            api_handler: SpotifyApiHandler::new(config, cache)?,
        })
    }

    fn get_playing_state(&self) -> Result<Option<PlayingState>, SpotifyError> {
        let state = self.api_handler.make_request(&GetPlaybackState).map_err(SpotifyError::ApiRequestError)?;
        Ok(state.map(|st| PlayingState::from(st.is_playing)))
    }

    fn verify_playing_state(&self, excpected_state: PlayingState) -> Result<(), SpotifyError> {
        match self.get_playing_state()? {
            Some(state) if state != excpected_state => Err(SpotifyError::UnwantedPlayingState(state)),
            Some(_) => Ok(()),
            None => Err(SpotifyError::NoActiveDevice),
        }
    }

    fn verify_active_device(&self) -> Result<(), SpotifyError> {
        self.get_playing_state()?.ok_or(SpotifyError::NoActiveDevice).map(|_| ())
    }
}
