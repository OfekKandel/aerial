use crate::{
    modules::music::Token,
    utils::{
        cache::SpotifyCache,
        http::{ExtractFromResposneExt, ResponseError, ValidateResponseExt},
        server::{read_localhost_request, TcpServerError},
        AuthClient, Cache,
    },
};
use base64::{engine::general_purpose::STANDARD as base64_engine, Engine as _};
use reqwest::{
    blocking::RequestBuilder,
    header::{AUTHORIZATION, CONTENT_TYPE},
};
use std::{
    collections::HashMap,
    time::{Duration, SystemTime},
};
use thiserror::Error;
use url::Url;

const AUTH_ENDPOINT: &str = "https://accounts.spotify.com";
const REDIRECT_PORT: u32 = 8888;
const API_SCOPE: &str = "user-read-playback-state user-modify-playback-state user-library-modify";

pub struct SpotifyAuthClient {
    token: Token,
}

impl AuthClient for SpotifyAuthClient {
    fn add_auth(&self, request: RequestBuilder) -> RequestBuilder {
        request.header(AUTHORIZATION, self.token.as_auth())
    }
}

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("You are unauthenticated, run `music auth` to to authenticate")]
    NeedsInitialAuth,
    #[error("Failed to refresh authentication token: {0}")]
    FailedTokenRefresh(ResponseError),
}

#[derive(Debug, Error)]
pub enum InitialAuthError {
    #[error("Failed to create a valid callback URL, this is usually a problem in the code: {0}")]
    FailedCallbackUrlCreation(url::ParseError),
    #[error("Failed to open the Spotify authorization window: {0}")]
    FailedToOpenAuthWindow(opener::OpenError),
    #[error("Failed to read the the redirect: {0}")]
    FailedToReadRedirect(TcpServerError),
    #[error("Code param not found in redirect, given params: {0:?}")]
    CodeNotFoundInRedirect(HashMap<String, String>),
    #[error("Failed to get a token: {0}")]
    FailedToGetToken(ResponseError),
}

impl SpotifyAuthClient {
    pub fn new(cache: &mut Cache, client_id: &str, client_secret: &str) -> Result<Self, AuthError> {
        let token = Self::auth(cache, client_id, client_secret)?;
        cache.modules.spotify = Some(SpotifyCache { token: token.clone() });
        Ok(Self { token })
    }

    pub fn remove_auth_from_cache(cache: &mut Cache) {
        cache.modules.spotify = None
    }

    pub fn add_auth_to_cache(cache: &mut Cache, client_id: &str, client_secret: &str) -> Result<(), InitialAuthError> {
        let token = Self::initial_auth(client_id, client_secret)?;
        cache.modules.spotify = Some(SpotifyCache { token: token.clone() });
        Ok(())
    }

    fn auth(cache: &Cache, client_id: &str, client_secret: &str) -> Result<Token, AuthError> {
        match Self::get_token_from_cache(cache) {
            Some(token) if token.is_valid() => Ok(token.clone()),
            Some(token) => Self::refresh_token(token, client_id, client_secret).map_err(AuthError::FailedTokenRefresh),
            None => Err(AuthError::NeedsInitialAuth),
        }
    }

    fn get_token_from_cache(cache: &Cache) -> Option<&Token> {
        Some(&cache.modules.spotify.as_ref()?.token)
    }

    fn refresh_token(prev_token: &Token, client_id: &str, client_secret: &str) -> Result<Token, ResponseError> {
        let encoded_auth = base64_engine.encode(format!("{}:{}", client_id, client_secret));
        let response = reqwest::blocking::Client::new()
            .post(format!("{}/api/token", AUTH_ENDPOINT))
            .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
            .header(AUTHORIZATION, format!("Basic {}", encoded_auth))
            .form(&[("grant_type", "refresh_token"), ("refresh_token", prev_token.refresh_token.as_str())])
            .send();
        Ok(response.validate()?.extract::<RefreshTokenFromApi>()?.to_token(prev_token))
    }

    fn initial_auth(client_id: &str, client_secret: &str) -> Result<Token, InitialAuthError> {
        Self::open_auth_window(client_id)?;
        let code = Self::get_code_from_callback()?;
        let token = Self::get_token(code, client_id, client_secret).map_err(InitialAuthError::FailedToGetToken)?;
        Ok(token)
    }

    fn open_auth_window(client_id: &str) -> Result<(), InitialAuthError> {
        let redirect_uri = format!("http://localhost:{}/callback", REDIRECT_PORT);
        let uri = Url::parse_with_params(
            format!("{}/{}", AUTH_ENDPOINT, "authorize").as_str(),
            &[
                ("response_type", "code"),
                ("client_id", client_id),
                ("redirect_uri", redirect_uri.as_str()),
                ("scope", API_SCOPE),
            ],
        )
        .map_err(InitialAuthError::FailedCallbackUrlCreation)?;

        opener::open(uri.to_string()).map_err(InitialAuthError::FailedToOpenAuthWindow)
    }

    fn get_code_from_callback() -> Result<String, InitialAuthError> {
        let callback = read_localhost_request(REDIRECT_PORT).map_err(InitialAuthError::FailedToReadRedirect)?;
        Ok(callback
            .params
            .get("code")
            .ok_or(InitialAuthError::CodeNotFoundInRedirect(callback.params.clone()))?
            .to_string())
    }

    fn get_token(code: String, client_id: &str, client_secret: &str) -> Result<Token, ResponseError> {
        let encoded_auth = base64_engine.encode(format!("{}:{}", client_id, client_secret));
        let redirect_uri = format!("http://localhost:{}/callback", REDIRECT_PORT);
        let response = reqwest::blocking::Client::new()
            .post(format!("{}/api/token", AUTH_ENDPOINT))
            .header(AUTHORIZATION, format!("Basic {}", encoded_auth))
            .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
            .form(&[
                ("grant_type", "authorization_code"),
                ("code", code.as_str()),
                ("redirect_uri", redirect_uri.as_str()),
            ])
            .send();

        Ok(response.validate()?.extract::<TokenFromApi>()?.into())
    }
}

#[derive(serde::Deserialize)]
struct TokenFromApi {
    access_token: String,
    token_type: String,
    expires_in: u64,
    refresh_token: String,
}

#[derive(serde::Deserialize)]
struct RefreshTokenFromApi {
    access_token: String,
    token_type: String,
    expires_in: u64,
    refresh_token: Option<String>,
}

impl From<TokenFromApi> for Token {
    fn from(value: TokenFromApi) -> Self {
        Self {
            access_token: value.access_token,
            token_type: value.token_type,
            expires_in: Duration::from_secs(value.expires_in),
            time_set: SystemTime::now(),
            refresh_token: value.refresh_token,
        }
    }
}

impl RefreshTokenFromApi {
    fn to_token(self, prev_token: &Token) -> Token {
        Token {
            access_token: self.access_token,
            token_type: self.token_type,
            expires_in: Duration::from_secs(self.expires_in),
            time_set: SystemTime::now(),
            refresh_token: self.refresh_token.unwrap_or(prev_token.refresh_token.clone()),
        }
    }
}
