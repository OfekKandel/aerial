use super::spotify_api_handler::SpotifyApiHandler;
use super::spotify_api_spec::{
    GetCurrentTrack, GetPlaybackState, GotoNextTrack, GotoPrevTrack, Pause, Play, PlaybackState, PlayingState, Resume, Search, SetShuffle,
    ShuffleState, SpotifySearchType,
};
use super::spotify_auth::{AuthError, InitialAuthError};
use crate::modules::music::spotify::spotify_api_spec::spotify_search_results_to_string;
use crate::modules::music::MusicClient;
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

    fn play(&self, id: Option<String>, context: Option<String>) -> Result<(), Self::Error> {
        self.verify_active_device()?;
        let request = match (context, id) {
            (Some(context), id) => Play::Context { uri: context, track: id },
            (None, Some(id)) => Play::Track { id },
            _ => {
                eprintln!("WARNING: Play called without id or context to play, this is a code problem\n");
                return Ok(());
            }
        };

        self.api_handler.make_request(&request).map_err(SpotifyError::ApiRequestError)?;
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

    fn set_shuffle_state(&self, state: &ShuffleState) -> Result<(), Self::Error> {
        let shuffle = state.into_bool();
        self.verify_active_device()?;
        self.api_handler
            .make_request(&SetShuffle { state: shuffle })
            .map_err(SpotifyError::ApiRequestError)?;
        Ok(())
    }

    fn search(&self, query: String, search_type: SpotifySearchType) -> Result<(), Self::Error> {
        let search_results = self
            .api_handler
            .make_request(&Search {
                query: query.clone(),
                search_type: vec![search_type.clone()],
            })
            .map_err(SpotifyError::ApiRequestError)?;
        println!(
            "{}",
            match search_type {
                SpotifySearchType::Track => spotify_search_results_to_string(search_results.tracks.map(|i| i.items)),
                SpotifySearchType::Album => spotify_search_results_to_string(search_results.albums.map(|i| i.items)),
                SpotifySearchType::Artist => spotify_search_results_to_string(search_results.artists.map(|i| i.items)),
                SpotifySearchType::Playlist => spotify_search_results_to_string(search_results.playlists.map(|i| i.items)),
            }
        );
        Ok(())
    }

    fn print_current_track(&self) -> Result<(), Self::Error> {
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
        let state: Option<PlaybackState> = self
            .api_handler
            .make_request(&GetPlaybackState)
            .map_err(SpotifyError::ApiRequestError)?
            .into();
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
