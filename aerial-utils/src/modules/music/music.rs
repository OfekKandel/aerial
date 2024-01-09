use super::{
    spotify_client::{SpotifyClient, SpotifyError},
    InitialAuthError, MusicClient,
};
use crate::modules::Module;
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
    /// Play (resume) the currently playing track
    Pause,
}

#[derive(Error, Debug)]
pub enum MusicError {
    #[error("Failed to retrieve token for API requests: {0}")]
    FailedInitialAuth(InitialAuthError),
    #[error("Failed to perform action: {0}")]
    FailedAction(SpotifyError),
}

pub struct Music {}

impl Module for Music {
    type Args = MusicArgs;
    type Error = MusicError;

    fn run_with_args(args: &Self::Args) -> Result<(), Self::Error> {
        let client = SpotifyClient::new().map_err(MusicError::FailedInitialAuth)?;
        match args.command {
            MusicCommands::Pause => client.pause(),
        }
        .map_err(MusicError::FailedAction)
    }
}

impl Display for Music {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Music")
    }
}
