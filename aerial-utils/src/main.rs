mod modules;
mod utils;

use clap::{Parser, Subcommand};
use modules::{music::MusicArgs, Module, Music};
use utils::Config;

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
}

fn run_module(module: Modules) -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_file(CONFIG_PATH)?;
    match &module {
        Modules::Music(args) => Ok(Music::run(args, config)?),
    }
}

fn main() {
    let args = AerialUtilsArgs::parse();
    match run_module(args.module) {
        Ok(_) => println!("Command performed succesfully"),
        Err(err) => eprintln!("MODULE FAILED: {}", err),
    }
}
