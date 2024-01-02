use super::MusicClient;
use crate::utils::http::*;
use reqwest::{self, header::CONTENT_TYPE};
use std::fmt::Display;

pub struct SpotifyClient {
    token: Token,
}

impl SpotifyClient {
    pub fn new() -> Result<Self, ResponseError> {
        Ok(Self {
            token: Self::get_token()?,
        })
    }
    fn get_token() -> Result<Token, ResponseError> {
        let request = reqwest::blocking::Client::new()
            .post("https://accounts.spotify.com/api/token")
            .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
            .form(&[
                ("grant_type", "client_credentials"),
                ("client_id", ""),
                ("client_secret", ""),
            ])
            .send();
        Ok(request.validate()?.extract()?)
    }
}

impl MusicClient for SpotifyClient {
    fn play(&self) {
        println!("{}", self.token.access_token)
    }
}

#[derive(serde::Deserialize)]
struct Token {
    access_token: String,
    token_type: String,
    expires_in: i32,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "token: {}\ntype: {}\nexpires_in: {}",
            self.access_token, self.token_type, self.expires_in
        )
    }
}
