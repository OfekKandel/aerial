use crate::impl_endpoint;
use crate::utils::api_spec::NoResponse;
use crate::utils::ApiRequest;
use crate::utils::ApiRequestSpec;
use reqwest::Method;
use serde::Deserialize;

pub struct Pause;
impl_endpoint!(Pause, Method::PUT, "me/player/pause", NoResponse);

pub struct Resume;
impl_endpoint!(Resume, Method::PUT, "me/player/play", NoResponse);

pub struct GotoNextTrack;
impl_endpoint!(GotoNextTrack, Method::POST, "me/player/next", NoResponse);

pub struct GotoPrevTrack;
impl_endpoint!(GotoPrevTrack, Method::POST, "me/player/previous", NoResponse);

pub struct GetPlaybackState;
impl_endpoint!(GetPlaybackState, Method::GET, "me/player", PlaybackState);

#[derive(Deserialize)]
pub struct PlaybackState {
    pub device: SpotifyDevice,
}

#[derive(Deserialize)]
pub struct SpotifyDevice {
    pub id: Option<String>,
    pub is_active: bool,
    pub name: String,
    #[serde(alias = "type")]
    pub device_type: String,
}
