use crate::utils::api_spec::NoResponse;
use std::error::Error;

pub trait MusicClient {
    type Error: Error;

    fn pause(&self) -> Result<NoResponse, Self::Error>;
    fn resume(&self) -> Result<NoResponse, Self::Error>;
    fn goto_next_track(&self) -> Result<NoResponse, Self::Error>;
    fn goto_prev_track(&self) -> Result<NoResponse, Self::Error>;
    fn print_curr_track(&self) -> Result<(), Self::Error>;
}
