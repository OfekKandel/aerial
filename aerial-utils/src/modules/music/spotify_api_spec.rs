use crate::impl_endpoint;
use crate::utils::api_spec::NoResponse;
use crate::utils::ApiRequest;
use crate::utils::ApiRequestSpec;
use reqwest::Method;
use serde::Deserialize;
use std::fmt::Display;

pub struct Pause;
impl_endpoint!(Pause, Method::PUT, "me/player/pause", NoResponse);

pub struct Resume;
impl_endpoint!(Resume, Method::PUT, "me/player/play", NoResponse);

pub struct GotoNextTrack;
impl_endpoint!(GotoNextTrack, Method::POST, "me/player/next", NoResponse);

pub struct GotoPrevTrack;
impl_endpoint!(GotoPrevTrack, Method::POST, "me/player/previous", NoResponse);

pub struct GetPlaybackState;
type PlaybackResponse = Option<PlaybackState>;
impl_endpoint!(GetPlaybackState, Method::GET, "me/player", PlaybackResponse);

#[derive(Deserialize)]
pub struct PlaybackState {
    pub device: SpotifyDevice,
    pub is_playing: bool,
}

#[derive(Deserialize)]
pub struct SpotifyDevice {
    pub id: Option<String>,
    pub is_active: bool,
    pub name: String,
    #[serde(alias = "type")]
    pub device_type: String,
}

pub struct GetCurrentTrack;
impl_endpoint!(GetCurrentTrack, Method::GET, "me/player/currently-playing", CurrentTrack);

#[derive(Deserialize)]
pub struct CurrentTrack {
    pub item: Option<SpotifyTrack>,
}

#[derive(Deserialize)]
pub struct SpotifyTrack {
    pub name: String,
    pub album: SpotifyPartialAlbum,
    pub artists: Vec<SpotifyPartialArtist>,
}

impl Display for SpotifyTrack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name_line = format!("Name: {}", self.name);
        let album_line = format!("Album: {}", self.album.name);
        let artist_names: Vec<&str> = self.artists.iter().map(|artist| artist.name.as_str()).collect();
        let artists_line = format!("Artist(s): {}", artist_names.join(", "));
        write!(f, "{}", [name_line, album_line, artists_line].join("\n"))
    }
}

#[derive(Deserialize)]
pub struct SpotifyPartialAlbum {
    pub name: String,
}

#[derive(Deserialize)]
pub struct SpotifyPartialArtist {
    pub name: String,
}

// Other types -------------------------------------------------
#[derive(Debug, PartialEq)]
pub enum PlayingState {
    Playing,
    Paused,
}

impl From<bool> for PlayingState {
    fn from(value: bool) -> Self {
        return if value { Self::Playing } else { Self::Paused };
    }
}

impl Display for PlayingState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayingState::Playing => write!(f, "playing"),
            PlayingState::Paused => write!(f, "paused"),
        }
    }
}
