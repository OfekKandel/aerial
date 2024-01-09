use std::error::Error;

pub trait MusicClient {
    type Error: Error;

    fn pause(&self) -> Result<(), Self::Error>;
}
