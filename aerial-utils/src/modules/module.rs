use std::fmt::Display;

use crate::utils::Config;

pub trait Module: Display {
    type Args: clap::Args;
    type Error: std::error::Error;

    fn run(args: &Self::Args, config: Config) -> Result<(), Self::Error>;
}
