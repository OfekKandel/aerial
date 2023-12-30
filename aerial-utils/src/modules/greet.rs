use clap::Args;

use crate::modules::Module;

#[derive(Args)]
pub struct GreetArgs {
    /// The name to greet
    name: String,
}

pub struct Greet {}

impl Module for Greet {
    type Args = GreetArgs;

    fn run_with_params(args: &Self::Args) {
        println!("Hello {}", args.name);
    }
}
