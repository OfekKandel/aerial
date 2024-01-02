use std::fmt::Display;

pub trait Module: Display {
    type Args: clap::Args;
    type Error: std::error::Error;

    fn run_with_args(args: &Self::Args) -> Result<(), Self::Error>;
}
