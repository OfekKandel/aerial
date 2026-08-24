use reqwest::{blocking::Response, StatusCode};
use serde::de::DeserializeOwned;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ResponseError {
    #[error("Got an invalid resposne:\n{0}")]
    InvalidResposne(ResponseValidationError),
    #[error("Couldn't extract data from response:\n{0}")]
    InvalidExtraction(ResponseExtractionError),
}

#[derive(Error, Debug)]
pub enum ResponseValidationError {
    #[error("Failed to send HTTP request:\n{0}")]
    FailedToSendRequest(reqwest::Error),
    #[error("Got a bad status code {0}, body:\n{1}")]
    BadStatusCode(StatusCode, String),
}

#[derive(Error, Debug)]
pub enum ResponseExtractionError {
    #[error("Failed to extract data from response JSON:\nSTATUS: {status}\nHEADERS:\n{headers}\nERROR: {error}\nBODY: {body}")]
    FailedToExtractFromJSON {
        status: StatusCode,
        headers: String,
        error: serde_json::Error,
        body: String,
    },
}

pub trait ValidateResponseExt {
    fn validate(self) -> Result<Response, ResponseValidationError>;
}

impl ValidateResponseExt for Result<Response, reqwest::Error> {
    fn validate(self) -> Result<Response, ResponseValidationError> {
        match self {
            Ok(response) => match response.status().as_u16() {
                200..=299 => Ok(response),
                _ => Err(ResponseValidationError::BadStatusCode(
                    response.status(),
                    response.text().unwrap_or("No body returned".to_string()),
                )),
            },
            Err(err) => Err(ResponseValidationError::FailedToSendRequest(err)),
        }
    }
}

pub trait ExtractFromResposneExt {
    fn extract<T: DeserializeOwned>(self) -> Result<T, ResponseExtractionError>;
}

impl ExtractFromResposneExt for Response {
    fn extract<T: DeserializeOwned>(self) -> Result<T, ResponseExtractionError> {
        let status = self.status();
        let headers = self
            .headers()
            .iter()
            .map(|(name, value)| format!("  {}: {}", name, value.to_str().unwrap_or("<non-utf8>")))
            .collect::<Vec<_>>()
            .join("\n");

        let text = self.text().unwrap_or_default();
        let body = text.trim();

        let primary_err = if body.is_empty() {
            None
        } else {
            match serde_json::from_str::<T>(body) {
                Ok(value) => return Ok(value),
                Err(err) => Some(err),
            }
        };

        // If T=NoResponse then this is fine, meaning the error gets ignored, else we return the error
        serde_json::from_str::<T>("{}").map_err(|fallback_err| ResponseExtractionError::FailedToExtractFromJSON {
            status,
            headers,
            error: primary_err.unwrap_or(fallback_err),
            body: text,
        })
    }
}

impl From<ResponseValidationError> for ResponseError {
    fn from(value: ResponseValidationError) -> Self {
        Self::InvalidResposne(value)
    }
}

impl From<ResponseExtractionError> for ResponseError {
    fn from(value: ResponseExtractionError) -> Self {
        Self::InvalidExtraction(value)
    }
}
