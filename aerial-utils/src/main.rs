mod modules;
mod utils;

use clap::{Parser, Subcommand};
use modules::{music::MusicArgs, Module, Music};

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
    match &module {
        Modules::Music(args) => Ok(Music::run_with_args(args)?),
    }
}

fn main() {
    let args = AerialUtilsArgs::parse();
    match run_module(args.module) {
        Ok(_) => println!("Command performed succesfully"),
        Err(err) => println!("Module failed: {}", err),
    }
}
