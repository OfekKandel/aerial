use std::error::Error;

pub trait MusicClient {
    type Error: Error;

    fn toggle(&self) -> Result<(), Self::Error>;
    fn pause(&self) -> Result<(), Self::Error>;
    fn resume(&self) -> Result<(), Self::Error>;
    fn goto_next_track(&self) -> Result<(), Self::Error>;
    fn goto_prev_track(&self) -> Result<(), Self::Error>;
    fn print_curr_track(&self) -> Result<(), Self::Error>;
}
