use super::{
    spotify_client::{InitialAuthError, SpotifyClient},
    MusicClient,
};
use crate::{modules::Module, utils::http::ResponseError};
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
    Play,
}

#[derive(Error, Debug)]
pub enum MusicError {
    #[error("Failed to retrieve token for API requests: {0}")]
    FailedInitialAuth(InitialAuthError),
    #[error("API request failed: {0}")]
    FailedRequest(ResponseError),
}

pub struct Music {}

impl Module for Music {
    type Args = MusicArgs;
    type Error = MusicError;

    fn run_with_args(args: &Self::Args) -> Result<(), Self::Error> {
        let client = SpotifyClient::new().map_err(MusicError::FailedInitialAuth)?;
        match args.command {
            MusicCommands::Play => client.play().map_err(MusicError::FailedRequest),
        }
    }
}

impl Display for Music {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Music")
    }
}
