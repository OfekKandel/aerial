mod modules;
use clap::{Parser, Subcommand};
use modules::{Greet, GreetArgs, Leave, LeaveArgs, Module};

#[derive(Parser)]
#[command(version)]
pub struct AerialUtilsArgs {
    /// Which util do you want to use
    #[command(subcommand)]
    module: Modules,
}

#[derive(Subcommand)]
enum Modules {
    Greet(GreetArgs),
    Leave(LeaveArgs),
}

fn main() {
    let args = AerialUtilsArgs::parse();
    match &args.module {
        Modules::Greet(args) => Greet::run_with_params(args),
        Modules::Leave(args) => Leave::run_with_params(args),
    }
}
