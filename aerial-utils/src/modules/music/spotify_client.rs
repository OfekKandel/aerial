use super::MusicClient;
use crate::utils::{
    http::*,
    server::{read_localhost_request, TcpServerError},
};
use base64::{
    engine::{self, general_purpose},
    Engine as _,
};
use reqwest::{
    self,
    blocking::Response,
    header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE},
};
use std::{collections::HashMap, fmt::Display};
use thiserror::Error;

const AUTH_ENDPOINT: &str = "https://accounts.spotify.com";
const API_ENDPOINT: &str = "https://api.spotify.com/v1";

#[derive(Debug, Error)]
pub enum InitialAuthError {
    #[error("Failed to open the Spotify authorization window: {0}")]
    FailedToOpenAuthWindow(opener::OpenError),
    #[error("Failed to read the the redirect: {0}")]
    FailedToReadRedirect(TcpServerError),
    #[error("Failed to read the code param from the redirect, given params: {0:?}")]
    FailedToReadCodeFromRedirect(HashMap<String, String>),
    #[error("Failed to get a token: {0}")]
    FailedToGetToken(ResponseError),
}

pub struct SpotifyClient {
    token: Token,
}

impl MusicClient for SpotifyClient {
    fn play(&self) -> Result<(), ResponseError> {
        Ok(self
            .put_request("me/player/pause".to_string())
            .map(|_| ())?)
    }
}

impl SpotifyClient {
    pub fn new() -> Result<Self, InitialAuthError> {
        Self::open_auth_window().map_err(InitialAuthError::FailedToOpenAuthWindow)?;
        let code = Self::get_code_from_callback()?;
        Ok(Self {
            token: Self::get_token(code, "".into(), "".into())
                .map_err(InitialAuthError::FailedToGetToken)?,
        })
    }

    fn open_auth_window() -> Result<(), opener::OpenError> {
        // TODO: Convert this to a URL element with params
        let uri = format!(
            "{}/{}?response_type=code&client_id={}&redirect_uri=http://localhost:8888/callback&scope={}",
            AUTH_ENDPOINT, "authorize", "", "user-modify-playback-state"
        );
        opener::open(uri)
    }

    fn get_code_from_callback() -> Result<String, InitialAuthError> {
        let params = read_localhost_request(8888, "/callback".to_string())
            .map_err(InitialAuthError::FailedToReadRedirect)?
            .params;
        Ok(params
            .get("code")
            .ok_or(InitialAuthError::FailedToReadCodeFromRedirect(
                params.clone(),
            ))?
            .to_string())
    }

    fn get_token(
        code: String,
        client_id: String,
        client_secret: String,
    ) -> Result<Token, ResponseError> {
        let encoded_auth =
            general_purpose::STANDARD.encode(format!("{}:{}", client_id, client_secret));
        let request = reqwest::blocking::Client::new()
            .post(format!("{}/api/token", AUTH_ENDPOINT))
            .header(AUTHORIZATION, format!("Basic {}", encoded_auth))
            .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
            .form(&[
                ("grant_type", "authorization_code"),
                ("code", code.as_str()),
                ("redirect_uri", "http://localhost:8888/callback"),
            ])
            .send();
        Ok(request.validate()?.extract()?)
    }

    fn put_request(&self, endpoint: String) -> Result<Response, ResponseError> {
        Ok(reqwest::blocking::Client::new()
            .put(format!("{}/{}", API_ENDPOINT, endpoint))
            .header(CONTENT_TYPE, "application/json")
            .header(AUTHORIZATION, self.token.as_auth())
            .header(CONTENT_LENGTH, 0)
            .send()
            .validate()?)
    }
}

#[derive(serde::Deserialize)]
struct Token {
    access_token: String,
    token_type: String,
    expires_in: i32,
}

impl Token {
    pub fn as_auth(&self) -> String {
        format!("{}  {}", self.token_type, self.access_token)
    }
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
