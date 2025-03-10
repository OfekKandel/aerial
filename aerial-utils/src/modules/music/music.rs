use super::{
    spotify::{
        spotify_api_spec::{ShuffleState, SpotifySearchType},
        spotify_client::{SpotifyClient, SpotifyError},
    },
    AuthError, MusicClient, SpotifyAuthClient,
};
use crate::{
    modules::Module,
    utils::{cache::Cache, config::SpotifyConfig, Config},
};
use clap::{Args, Subcommand};
use std::fmt::Display;
use thiserror::Error;

#[derive(Args)]
pub struct MusicArgs {
    #[command(subcommand)]
    command: MusicCommands,
}

#[derive(Subcommand)]
pub enum MusicCommands {
    /// Pause the music if it's playing, resume it if it's paused
    Toggle,
    /// Pause the currently playing track
    Pause,
    /// Resume the currently playing track
    Resume,
    /// Play a Spotify track
    Play(PlayArgs),
    /// Go to the next track
    Next,
    /// Go to the previous track
    Prev,
    /// Get a list of tracks for a given query
    Search {
        /// The search query
        query: String,
        /// The type of results to be searched for
        #[clap(short = 't', long, default_value_t, value_enum)]
        search_type: SpotifySearchType
    },
    /// Sets the shuffle state to the given parameter
    SetShuffle {
        /// Weather to turn shuffle on or off
        state: ShuffleState,
    },
    /// Add a track to the user's 'Liked Songs' playlist
    Save {
        /// The ids of the tracks to save
        #[arg(num_args = 1..)]
        ids: Vec<String>
    },
    /// Print information about the current track
    CurrTrack,
    /// Initialize authentication to Spotify
    Auth,
    /// Remove authentication to Spotify
    Unauth,
}

#[derive(Args)]
#[group(required = true, multiple = true)]
pub struct PlayArgs {
    /// The spotify track id to play
    #[arg(short, long)]
    track: Option<String>,
    /// The spotify context to play in, formated as album:album_id or playlist:playlist_id
    #[arg(short, long)]
    context: Option<String>,
}

#[derive(Error, Debug)]
pub enum MusicError {
    #[error("No configuration for Spotify found in the config file")]
    MissingConfig,
    #[error("Failed to perform action: {0}")]
    FailedAction(SpotifyError),
    #[error("Failed to authenticate to API: {0}")]
    FailedAuth(AuthError),
}

pub struct Music {}

impl Music {
    fn generate_client(config: &SpotifyConfig, cache: &mut Cache) -> Result<SpotifyClient, MusicError> {
        Ok(SpotifyClient::new(config, cache).map_err(MusicError::FailedAuth)?)
    }
}

impl Module for Music {
    type Args = MusicArgs;
    type Error = MusicError;

    fn run(args: Self::Args, config: &Config, cache: &mut Cache) -> Result<(), Self::Error> {
        let spotify_config = config.modules.spotify.as_ref().ok_or(MusicError::MissingConfig)?;

        // TODO: Remove all of these generate_client calls
        match args.command {
            MusicCommands::Auth => {
                SpotifyAuthClient::add_auth_to_cache(cache, &spotify_config.client_id.as_str(), &spotify_config.client_secret.as_str())
                    .map_err(SpotifyError::FailedInitialAuth)
            }
            MusicCommands::Toggle => Self::generate_client(spotify_config, cache)?.toggle(),
            MusicCommands::Pause => Self::generate_client(spotify_config, cache)?.pause(),
            MusicCommands::Resume => Self::generate_client(spotify_config, cache)?.resume(),
            MusicCommands::Play(args) => Self::generate_client(spotify_config, cache)?.play(args.track, args.context),
            MusicCommands::Next => Self::generate_client(spotify_config, cache)?.goto_next_track(),
            MusicCommands::Prev => Self::generate_client(spotify_config, cache)?.goto_prev_track(),
            MusicCommands::Unauth => Ok(SpotifyAuthClient::remove_auth_from_cache(cache)),
            MusicCommands::SetShuffle { state } => Self::generate_client(&spotify_config, cache)?.set_shuffle_state(&state),
            MusicCommands::Save { ids } => Self::generate_client(spotify_config, cache)?.save_tracks(ids),
            MusicCommands::Search { query, search_type } => Self::generate_client(&spotify_config, cache)?.search(query.clone(), search_type),
            MusicCommands::CurrTrack => Self::generate_client(&spotify_config, cache)?.print_current_track(),
        }
        .map_err(MusicError::FailedAction)
    }
}

impl Display for Music {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Music")
    }
}
