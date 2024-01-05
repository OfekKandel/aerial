use crate::utils::http::ResponseError;

pub trait MusicClient {
    fn play(&self) -> Result<(), ResponseError>;
}
