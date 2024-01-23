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
