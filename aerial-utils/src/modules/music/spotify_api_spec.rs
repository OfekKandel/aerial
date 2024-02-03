use crate::impl_endpoint;
use crate::utils::api_spec::NoBody;
use crate::utils::api_spec::NoResponse;
use crate::utils::api_spec::OptionalResponse;
use crate::utils::ApiRequest;
use crate::utils::ApiRequestSpec;
use clap::ValueEnum;
use reqwest::Method;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::Display;

pub struct Pause;
impl_endpoint!(Pause, Method::PUT, "me/player/pause", NoResponse);

pub struct Resume;
impl_endpoint!(Resume, Method::PUT, "me/player/play", NoResponse);

pub struct PlayTrack {
    pub id: String,
}
impl_endpoint!(PlayTrack, Method::PUT, "me/player/play", playtrack_body => PlayTrackBody, NoResponse);


#[derive(Serialize)]
pub struct PlayTrackBody {
    uris: Vec<String>,
}
fn playtrack_body(args: &PlayTrack) -> PlayTrackBody {
    let track_uri = format!("spotify:track:{}", args.id);
    PlayTrackBody { uris: vec![track_uri] }
}

pub struct GotoNextTrack;
impl_endpoint!(GotoNextTrack, Method::POST, "me/player/next", NoResponse);

pub struct GotoPrevTrack;
impl_endpoint!(GotoPrevTrack, Method::POST, "me/player/previous", NoResponse);

pub struct SetShuffle {
    pub state: bool,
}

impl_endpoint!(SetShuffle, Method::PUT, "me/player/shuffle" => setshuffle_params, NoResponse);
fn setshuffle_params(args: &SetShuffle) -> HashMap<String, String> {
    [("state".into(), args.state.to_string())].into()
}

#[derive(Clone, ValueEnum, Copy)]
pub enum ShuffleState {
    On,
    Off,
}

impl ShuffleState {
    pub fn into_bool(&self) -> bool {
        match self {
            ShuffleState::On => true,
            ShuffleState::Off => false,
        }
    }
}

pub struct GetPlaybackState;
type PlaybackResponse = OptionalResponse<PlaybackState>;
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
