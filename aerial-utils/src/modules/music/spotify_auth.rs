use crate::utils::{
    http::*,
    server::{read_localhost_request, TcpServerError},
};
use base64::{engine, Engine as _};
use reqwest::{
    self,
    blocking::Response,
    header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE},
    Url,
};
use std::{collections::HashMap, fmt::Display};
use thiserror::Error;
use url::ParseError;

const AUTH_ENDPOINT: &str = "https://accounts.spotify.com";
const API_ENDPOINT: &str = "https://api.spotify.com/v1";
const REDIRECT_PORT: u32 = 8888;
const API_SCOPE: &str = "user-modify-playback-state";

#[derive(Debug, Error)]
pub enum InitialAuthError {
    #[error("Failed to create a valid callback URL, this is usually a problem in the code: {0}")]
    FailedCallbackUrlCreation(ParseError),
    #[error("Failed to open the Spotify authorization window: {0}")]
    FailedToOpenAuthWindow(opener::OpenError),
    #[error("Failed to read the the redirect: {0}")]
    FailedToReadRedirect(TcpServerError),
    #[error("Failed to read the code param from the redirect, given params: {0:?}")]
    FailedToReadCodeFromRedirect(HashMap<String, String>),
    #[error("Failed to get a token: {0}")]
    FailedToGetToken(ResponseError),
}

pub struct SpotifyAuthClient {
    token: Token,
}

impl SpotifyAuthClient {
    pub fn new(client_id: &str, client_secret: &str) -> Result<Self, InitialAuthError> {
        Self::open_auth_window(client_id)?;
        let code = Self::get_code_from_callback()?;
        let token = Self::get_token(code, client_id, client_secret)
            .map_err(InitialAuthError::FailedToGetToken)?;
        Ok(Self { token })
    }

    fn open_auth_window(client_id: &str) -> Result<(), InitialAuthError> {
        let uri = Url::parse_with_params(
            format!("{}/{}", AUTH_ENDPOINT, "authorize").as_str(),
            &[
                ("response_type", "code"),
                ("client_id", client_id),
                (
                    "redirect_uri",
                    format!("http://localhost:{}/callback", REDIRECT_PORT).as_str(),
                ),
                ("scope", API_SCOPE),
            ],
        )
        .map_err(InitialAuthError::FailedCallbackUrlCreation)?;

        opener::open(uri.to_string()).map_err(InitialAuthError::FailedToOpenAuthWindow)
    }

    fn get_code_from_callback() -> Result<String, InitialAuthError> {
        let params = read_localhost_request(REDIRECT_PORT)
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
        client_id: &str,
        client_secret: &str,
    ) -> Result<Token, ResponseError> {
        let encoded_auth =
            engine::general_purpose::STANDARD.encode(format!("{}:{}", client_id, client_secret));
        let request = reqwest::blocking::Client::new()
            .post(format!("{}/api/token", AUTH_ENDPOINT))
            .header(AUTHORIZATION, format!("Basic {}", encoded_auth))
            .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
            .form(&[
                ("grant_type", "authorization_code"),
                ("code", code.as_str()),
                (
                    "redirect_uri",
                    format!("http://localhost:{}/callback", REDIRECT_PORT).as_str(),
                ),
            ])
            .send();
        Ok(request.validate()?.extract()?)
    }

    pub fn put_request(&self, endpoint: &str) -> Result<Response, ResponseError> {
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
