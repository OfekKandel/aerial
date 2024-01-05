use reqwest::Url;
use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpListener;
use std::net::TcpStream;

#[derive(Debug, thiserror::Error)]
pub enum TcpServerError {
    #[error("Failed to create TCP listener: {0}")]
    FailedToCreateTcpListener(io::Error),
    #[error("Failed to get a stream for a request: {0}")]
    FailedToGetStream(io::Error),
    #[error("Failed parse the request made to the server: {0}")]
    FailedToParseRequest(RequestFromStringError),
}

pub struct Request {
    pub request_type: String,
    pub path: String,
    pub params: HashMap<String, String>,
}

#[derive(Debug, thiserror::Error)]
pub enum RequestFromStringError {
    #[error("The given string")]
    StringIsEmpty,
    #[error("The first line could not be parsed: {0}")]
    InvalidFirstLine(String),
    #[error("The path the request was submitted to couldn't be parsed: {0}")]
    InvalidPath(String),
}

impl TryFrom<Vec<String>> for Request {
    type Error = RequestFromStringError;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        let request = value.first().ok_or(RequestFromStringError::StringIsEmpty)?;
        let request_parts: Vec<&str> = request.split_whitespace().collect();
        if request_parts.len() != 3 {
            return Err(RequestFromStringError::StringIsEmpty);
        }
        let request_type = request_parts[0];
        let url_str = format!("https://example.com{}", request_parts[1]);
        let url = Url::try_from(url_str.as_str())
            .map_err(|_| RequestFromStringError::InvalidPath(url_str))?;
        Ok(Self {
            request_type: request_type.into(),
            path: url.path().into(),
            params: url.query_pairs().into_owned().collect(),
        })
    }
}

pub fn read_localhost_request(port: u16, path: String) -> Result<Request, TcpServerError> {
    let listener = TcpListener::bind(format!("localhost:{}", port))
        .map_err(TcpServerError::FailedToCreateTcpListener)?;
    let (stream, _) = listener
        .accept()
        .map_err(TcpServerError::FailedToGetStream)?;
    handle_requset_stream(stream)
}

fn handle_requset_stream(mut stream: TcpStream) -> Result<Request, TcpServerError> {
    let request: Vec<_> = BufReader::new(&mut stream)
        .lines()
        .map(|result| result.unwrap_or("".into()))
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", request);
    write_close_tab_msg(stream);
    Request::try_from(request).map_err(TcpServerError::FailedToParseRequest)
}

fn write_close_tab_msg(mut stream: TcpStream) {
    let content = "<h1>You can close this tab now</h1>";
    let status_line = "HTTP/1.1 200 OK";
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        content.len(),
        content
    );
    stream.write_all(response.as_bytes()).unwrap();
}
