use super::{
    spotify::{
        spotify_api_spec::{ShuffleState, SpotifySearchType, SpotifyTimeRange},
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
        search_type: SpotifySearchType,
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
        ids: Vec<String>,
    },
    /// The user's top tracks
    TopTracks {
        #[clap(short = 't', long, default_value_t, value_enum)]
        time_range: SpotifyTimeRange,
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

        if let MusicCommands::Auth = args.command {
            return SpotifyAuthClient::add_auth_to_cache(cache, &spotify_config.client_id.as_str(), &spotify_config.client_secret.as_str())
                .map_err(SpotifyError::FailedInitialAuth)
                .map_err(MusicError::FailedAction);
        }

        let music_client = Self::generate_client(spotify_config, cache)?;

        match args.command {
            MusicCommands::Toggle => music_client.toggle(),
            MusicCommands::Pause => music_client.pause(),
            MusicCommands::Resume => music_client.resume(),
            MusicCommands::Play(args) => music_client.play(args.track, args.context),
            MusicCommands::Next => music_client.goto_next_track(),
            MusicCommands::Prev => music_client.goto_prev_track(),
            MusicCommands::Unauth => Ok(SpotifyAuthClient::remove_auth_from_cache(cache)),
            MusicCommands::SetShuffle { state } => music_client.set_shuffle_state(&state),
            MusicCommands::Save { ids } => music_client.save_tracks(ids),
            MusicCommands::TopTracks { time_range } => music_client.get_top_tracks(time_range),
            MusicCommands::Search { query, search_type } => music_client.search(query.clone(), search_type),
            MusicCommands::CurrTrack => music_client.print_current_track(),
            _ => unreachable!(),
        }
        .map_err(MusicError::FailedAction)
    }
}

impl Display for Music {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Music")
    }
}
