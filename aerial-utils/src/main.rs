mod modules;
mod utils;

use clap::{Parser, Subcommand};
use modules::{music::MusicArgs, print_subcommand_specs, Module, Music};
use utils::{cache::Cache, Config};

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

// TODO: Make this a normal error
fn run_module(module: Modules) -> Result<(), Box<dyn std::error::Error>> {
    let mut cache = Cache::from_file("cache.toml")?;
    let config = Config::from_file(CONFIG_PATH)?;
    let res = match module {
        Modules::Music(args) => Ok(Music::run(args, &config, &mut cache)?),
        Modules::CommandSpecs => {
            print_subcommand_specs();
            Ok(())
        }
    };
    // NOTE: Cache won't be changed if the operation failed, might be good because
    // running the same command twice shouldn't get a different result
    cache.to_file("cache.toml")?;
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
