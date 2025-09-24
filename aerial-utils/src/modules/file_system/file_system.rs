use std::{fmt::Display, io, path::PathBuf};

use clap::{Args, Subcommand};
use thiserror::Error;

use crate::modules::Module;
use super::basic_operations::*;

#[derive(Args)]
pub struct FileSystemArgs {
    #[command[subcommand]]
    command: FileSystemCommands,
}

#[derive(Subcommand)]
pub enum FileSystemCommands {
    /// List all the files in a directory
    #[command(visible_alias = "ls")]
    ListDirectoryContents {
        /// The path to the directory
        directorh_path: PathBuf,
    },
    /// List all the files in a directory
    #[command(visible_alias = "cat")]
    ReadFilePlaintext {
        /// The path to the file
        file_path: PathBuf,
    },
}

#[derive(Error, Debug)]
pub enum FileSystemError {
    #[error("IO Error:\n{0}")]
    IOError(io::Error),
}

pub struct FileSystem {}

impl Module for FileSystem {
    type Args = FileSystemArgs;
    type Error = FileSystemError;

    fn run(args: Self::Args, _config: &crate::utils::Config, _cache: &mut crate::utils::Cache) -> Result<(), Self::Error> {
        match args.command {
            FileSystemCommands::ListDirectoryContents { directorh_path } => list_dir(directorh_path).map_err(FileSystemError::IOError),
            FileSystemCommands::ReadFilePlaintext { file_path } => read_file_plaintext(file_path).map_err(FileSystemError::IOError),
        }
    }
}

impl Display for FileSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "File System")
    }
}
