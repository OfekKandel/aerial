use super::spotify::spotify_api_spec::{ShuffleState, SpotifySearchType};
use std::error::Error;

pub trait MusicClient {
    type Error: Error;

    fn toggle(&self) -> Result<(), Self::Error>;
    fn pause(&self) -> Result<(), Self::Error>;
    fn resume(&self) -> Result<(), Self::Error>;
    fn play(&self, track_id: Option<String>, context: Option<String>) -> Result<(), Self::Error>;
    fn goto_next_track(&self) -> Result<(), Self::Error>;
    fn goto_prev_track(&self) -> Result<(), Self::Error>;
    fn set_shuffle_state(&self, state: &ShuffleState) -> Result<(), Self::Error>;
    fn search(&self, query: String, search_type: SpotifySearchType) -> Result<(), Self::Error>;
    fn save_tracks(&self, ids: Vec<String>) -> Result<(), Self::Error>;
    fn print_current_track(&self) -> Result<(), Self::Error>;
}
