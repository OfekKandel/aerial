mod modules;
mod utils;

use clap::{Parser, Subcommand};
use thiserror::Error;

use modules::{
    file_system::{FileSystem, FileSystemArgs, FileSystemError},
    music::{Music, MusicArgs, MusicError},
    print_subcommand_specs, Module,
};
use utils::{
    cache::{Cache, CacheError},
    config::{Config, ConfigError},
};

const CONFIG_PATH: &str = "./config.toml";

#[derive(Parser)]
#[command(version)]
pub struct AerialUtilsArgs {
    #[command(subcommand)]
    module: Modules,
}

#[derive(Subcommand)]
enum Modules {
    /// The music module
    Music(MusicArgs),
    /// The file system module
    #[command(visible_alias = "fs")]
    FileSystem(FileSystemArgs),
    /// Print ChatGPT command specifications
    CommandSpecs,
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Cache error:\n{0}")]
    CacheError(CacheError),
    #[error("Config error:\n{0}")]
    ConfigError(ConfigError),
    #[error("Music module error:\n{0}")]
    MusicError(MusicError),
    #[error("File system module error:\n{0}")]
    FileSystemError(FileSystemError),
}

fn run_module(module: Modules) -> Result<(), AppError> {
    let mut cache = Cache::from_file("cache.toml").map_err(AppError::CacheError)?;
    let config = Config::from_file(CONFIG_PATH).map_err(AppError::ConfigError)?;
    let res = match module {
        Modules::Music(args) => Music::run(args, &config, &mut cache).map_err(AppError::MusicError),
        Modules::CommandSpecs => {
            print_subcommand_specs();
            Ok(())
        }
        Modules::FileSystem(args) => FileSystem::run(args, &config, &mut cache).map_err(AppError::FileSystemError),
    };
    // NOTE: Cache won't be changed if the operation failed, might be good because
    // running the same command twice shouldn't get a different result
    cache.to_file("cache.toml").map_err(AppError::CacheError)?;
    return res;
}

fn main() {
    let args = AerialUtilsArgs::parse();
    match run_module(args.module) {
        // Ok(_) => println!("Command performed succesfully"),
        Ok(_) => (),
        Err(err) => eprintln!("MODULE FAILED:\n{}", err),
    }
}
