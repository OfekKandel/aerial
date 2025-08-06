use crate::impl_endpoint;
use crate::utils::api_spec::{NoBody, NoResponse, OptionalResponse};
use crate::utils::ApiRequest;
use crate::utils::ApiRequestSpec;
use clap::ValueEnum;
use reqwest::Method;
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;
use std::fmt::Display;
use std::u8;

pub struct Pause;
impl_endpoint!(Pause, Method::PUT, "me/player/pause", NoResponse);

pub struct Resume;
impl_endpoint!(Resume, Method::PUT, "me/player/play", NoResponse);

pub enum Play {
    Track { id: String },
    Context { uri: String, track: Option<String> },
}
impl_endpoint!(Play, Method::PUT, "me/player/play", playtrack_body => PlayBody, NoResponse);

#[derive(Serialize, Default)]
pub struct PlayBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    uris: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    context_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<PlayBodyOffset>,
}
#[derive(Serialize)]
pub struct PlayBodyOffset {
    uri: String,
}
fn playtrack_body(args: &Play) -> PlayBody {
    match args {
        Play::Track { id } => PlayBody {
            uris: Some(vec![format!("spotify:track:{}", id)]),
            ..Default::default()
        },
        Play::Context { uri, track } => PlayBody {
            context_uri: Some(format!("spotify:{}", uri)),
            offset: track.as_ref().map(|id| PlayBodyOffset {
                uri: format!("spotify:track:{}", id),
            }),
            ..Default::default()
        },
    }
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

pub struct Search {
    pub query: String,
    pub search_type: Vec<SpotifySearchType>,
}

impl_endpoint!(Search, Method::GET, "search" => search_params, SpotifySearchResults);
fn search_params(args: &Search) -> HashMap<String, String> {
    [
        ("q".into(), args.query.clone()),
        (
            "type".into(),
            args.search_type.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(","),
        ),
    ]
    .into()
}

#[derive(clap::ValueEnum, Default, Clone, Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SpotifySearchType {
    #[default]
    Track,
    Album,
    Artist,
    Playlist,
}

impl Display for SpotifySearchType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            SpotifySearchType::Track => "track",
            SpotifySearchType::Album => "album",
            SpotifySearchType::Artist => "artist",
            SpotifySearchType::Playlist => "playlist",
        };
        write!(f, "{}", text)
    }
}

#[derive(Deserialize)]
pub struct SpotifySearchResults {
    pub tracks: Option<SpotifySearchTracksResults>,
    pub albums: Option<SpotifySearchAlbumsResults>,
    pub artists: Option<SpotifySearchArtistsResults>,
    pub playlists: Option<SpotifySearchPlaylistsResults>,
}

#[derive(Deserialize)]
pub struct SpotifySearchTracksResults {
    #[serde(alias = "total")]
    pub tracks_num: u32,
    #[serde(deserialize_with = "deserialize_vec_skip_null")]
    pub items: Vec<SpotifyTrack>,
}

#[derive(Deserialize)]
pub struct SpotifySearchAlbumsResults {
    #[serde(alias = "total")]
    pub tracks_num: u32,
    #[serde(deserialize_with = "deserialize_vec_skip_null")]
    pub items: Vec<SpotifySimplifiedAlbum>,
}

#[derive(Deserialize)]
pub struct SpotifySearchArtistsResults {
    #[serde(alias = "total")]
    pub tracks_num: u32,
    #[serde(deserialize_with = "deserialize_vec_skip_null")]
    pub items: Vec<SpotifyArtist>,
}

#[derive(Deserialize)]
pub struct SpotifySearchPlaylistsResults {
    #[serde(alias = "total")]
    pub tracks_num: u32,
    #[serde(deserialize_with = "deserialize_vec_skip_null")]
    pub items: Vec<SpotifySimplifiedPlaylist>,
}

pub fn spotify_search_results_to_string<T: Display>(results: Option<Vec<T>>) -> String {
    match results {
        Some(items) => items.iter().map(ToString::to_string).collect::<Vec<_>>().join("\n\n"),
        None => "No search results returned".into(),
    }
}

pub struct SaveTracks {
    pub ids: Vec<String>,
}
impl_endpoint!(SaveTracks, Method::PUT, "me/tracks", savetrack_body => SaveTrackBody, NoResponse);
#[derive(Serialize)]
pub struct SaveTrackBody {
    pub ids: Vec<String>,
}

fn savetrack_body(args: &SaveTracks) -> SaveTrackBody {
    SaveTrackBody { ids: args.ids.clone() }
}

pub struct GetTopTracks {
    pub time_range: SpotifyTimeRange,
}
impl_endpoint!(GetTopTracks, Method::GET, "me/top/tracks" => get_top_tracks_params, TopTracksResponse);
fn get_top_tracks_params(args: &GetTopTracks) -> HashMap<String, String> {
    [("time_range".into(), args.time_range.to_string())].into()
}

#[derive(clap::ValueEnum, Default, Clone)]
pub enum SpotifyTimeRange {
    Short, // ~4 Weeks
    #[default]
    Medium, // ~6 Months
    Long,  // ~1 year
}

impl ToString for SpotifyTimeRange {
    fn to_string(&self) -> String {
        match self {
            SpotifyTimeRange::Short => "short_term",
            SpotifyTimeRange::Medium => "medium_term",
            SpotifyTimeRange::Long => "long_term",
        }.into()
    }
}

#[derive(Deserialize)]
pub struct TopTracksResponse {
    pub items: Vec<SpotifyTrack>,
    pub total: u32,
    pub limit: u32,
    pub offset: u32,
    pub href: String,
    // TODO: Implement option to actually use this
    pub previous: Option<String>,
    pub next: Option<String>,
}

pub struct GetCurrentTrack;
impl_endpoint!(GetCurrentTrack, Method::GET, "me/player/currently-playing", CurrentTrack);

#[derive(Deserialize)]
pub struct CurrentTrack {
    pub item: Option<SpotifyTrack>,
    pub context: Option<SpotifyContext>,
}

impl Display for CurrentTrack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let track_info = match &self.item {
            Some(track) => format!("{}", track),
            None => "No track information available".to_string(),
        };

        let context_info = match &self.context {
            Some(context) => format!("CONTEXT:\n{}", context),
            None => "No context information available".to_string(),
        };

        write!(f, "{}\n\n{}", track_info, context_info)
    }
}

// Other types -------------------------------------------------
#[derive(Deserialize)]
pub struct SpotifySimplifiedArtist {
    pub name: String,
}

#[derive(Deserialize)]
pub struct SpotifySimplifiedPlaylist {
    pub name: String,
    pub description: String,
    pub id: String,
    #[serde(alias = "public")]
    pub is_public: bool,
}

impl Display for SpotifySimplifiedPlaylist {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lines = [
            format!("Name: {}", self.name),
            format!("Description: {}", self.description),
            format!("ID: {}", self.id),
            format!("Public: {}", if self.is_public { "Yes" } else { "No" }),
        ];
        write!(f, "{}", lines.join("\n"))
    }
}

#[derive(Deserialize)]
pub struct SpotifyArtist {
    pub name: String,
    pub genres: Vec<String>,
    pub id: String,
}

impl Display for SpotifyArtist {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lines = [
            format!("Name: {}", self.name),
            format!("Genres: {}", self.genres.join(", ")),
            format!("ID: {}", self.id),
        ];
        write!(f, "{}", lines.join("\n"))
    }
}

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

#[derive(Deserialize)]
pub struct SpotifyTrack {
    pub name: String,
    pub album: SpotifySimplifiedAlbum,
    pub artists: Vec<SpotifySimplifiedArtist>,
    pub id: String,
}

impl Display for SpotifyTrack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let artist_names: Vec<&str> = self.artists.iter().map(|artist| artist.name.as_str()).collect();
        let lines = [
            format!("Name: {}", self.name),
            format!("Album: {} (ID = {})", self.album.name, self.album.id),
            format!("Artist(s): {}", artist_names.join(", ")),
            format!("ID: {}", self.id),
        ];
        write!(f, "{}", lines.join("\n"))
    }
}

#[derive(Deserialize)]
pub struct SpotifyContext {
    pub uri: String,
    pub href: String,
    pub external_urls: HashMap<String, String>,
    #[serde(alias = "type")]
    pub context_type: String,
}

impl Display for SpotifyContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lines = [
            format!("URI: {}", self.uri),
            format!("Href: {}", self.href),
            format!("External URLs: {:?}", self.external_urls),
            format!("Context Type: {}", self.context_type),
        ];
        write!(f, "{}", lines.join("\n"))
    }
}

#[derive(Deserialize)]
pub struct SpotifySimplifiedAlbum {
    pub name: String,
    pub album_type: String, // TODO: Convert to Enum
    pub total_tracks: u8,
    pub artists: Vec<SpotifySimplifiedArtist>,
    pub release_date: String,
    pub release_date_precision: String,
    pub id: String,
}

impl Display for SpotifySimplifiedAlbum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let artist_names: Vec<&str> = self.artists.iter().map(|artist| artist.name.as_str()).collect();
        let lines = [
            format!("Name: {}", self.name),
            format!("Album: {}", self.album_type),
            format!("Length: {} Tracks", self.total_tracks),
            format!("Artist(s): {}", artist_names.join(", ")),
            format!("Release Date: {}", self.release_date),
            format!("ID: {}", self.id),
        ];
        write!(f, "{}", lines.join("\n"))
    }
}

// Serde help functions
// TODO: ORGANIZE
fn deserialize_vec_skip_null<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    let opt_vec: Option<Vec<Option<T>>> = Deserialize::deserialize(deserializer)?;
    Ok(opt_vec.unwrap_or_default().into_iter().flatten().collect())
}
