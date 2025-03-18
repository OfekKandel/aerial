mod modules;
mod utils;

use clap::{Parser, Subcommand};
use modules::{music::{MusicArgs, MusicError}, print_subcommand_specs, Module, Music};
use thiserror::Error;
use utils::{cache::{Cache, CacheError}, config::ConfigError, Config};

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
    /// Print ChatGPT command specifications
    CommandSpecs,
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Cache error: {0}")]
    CacheError(CacheError),
    #[error("Config error: {0}")]
    ConfigError(ConfigError),
    #[error("Music module error: {0}")]
    MusicError(MusicError),
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
        Err(err) => eprintln!("MODULE FAILED: {}", err),
    }
}
