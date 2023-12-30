use clap::Args;

use crate::modules::Module;

#[derive(Args)]
pub struct LeaveArgs {
    /// The name to greet
    name: String,
}

pub struct Leave {}

impl Module for Leave {
    type Args = LeaveArgs;

    fn run_with_params(args: &Self::Args) {
        println!("Bye {}", args.name);
    }
}
