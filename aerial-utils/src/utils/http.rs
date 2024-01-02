use reqwest::{blocking::Response, StatusCode};
use serde::de::DeserializeOwned;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ResponseError {
    #[error("Got an invalid resposne: {0}")]
    InvalidResposne(ResponseValidationError),
    #[error("Couldn't extract data from response: {0}")]
    InvalidExtraction(ResponseExtractionError),
}

#[derive(Error, Debug)]
pub enum ResponseValidationError {
    #[error("Failed to send HTTP request: {0}")]
    FailedToSendRequest(reqwest::Error),
    #[error("Got a bad status code {0}, body:\n{1}")]
    BadStatusCode(StatusCode, String),
}

#[derive(Error, Debug)]
pub enum ResponseExtractionError {
    #[error("Failed to extract data from response JSON:\n{0}")]
    FailedToExtractFromJSON(reqwest::Error),
}

pub trait ValidateResponseExt {
    fn validate(self) -> Result<Response, ResponseValidationError>;
}

impl ValidateResponseExt for Result<Response, reqwest::Error> {
    fn validate(self) -> Result<Response, ResponseValidationError> {
        match self {
            Ok(response) => match response.status() {
                reqwest::StatusCode::OK => Ok(response),
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
        self.json()
            .map_err(ResponseExtractionError::FailedToExtractFromJSON)
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
