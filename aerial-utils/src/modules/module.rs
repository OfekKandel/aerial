use crate::utils::{Cache, Config};
use std::fmt::Display;

pub trait Module: Display {
    type Args: clap::Args;
    type Error: std::error::Error;

    fn run(args: &Self::Args, config: &Config, cache: &mut Cache) -> Result<(), Self::Error>;
}
