use super::{spotify_client::SpotifyClient, MusicClient};
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
    FailedTokenCreation(ResponseError),
}

pub struct Music {}

impl Module for Music {
    type Args = MusicArgs;
    type Error = MusicError;

    fn run_with_args(args: &Self::Args) -> Result<(), Self::Error> {
        let client = SpotifyClient::new().map_err(MusicError::FailedTokenCreation)?;
        match args.command {
            MusicCommands::Play => Ok(client.play()),
        }
    }
}

impl Display for Music {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Music")
    }
}
