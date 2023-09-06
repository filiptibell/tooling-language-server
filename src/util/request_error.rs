use std::{fmt, string::FromUtf8Error};

use bytes::Bytes;
use reqwest::StatusCode;

pub type RequestResult<T, E = RequestError> = Result<T, E>;

#[derive(Debug, Clone)]
pub struct RequestError(String);

impl RequestError {
    pub fn is_not_found_error(&self) -> bool {
        let message = self.0.as_str().to_ascii_lowercase();
        message.contains("404") || message.contains("not found")
    }

    pub fn is_rate_limit_error(&self) -> bool {
        let message = self.0.as_str().to_ascii_lowercase();
        message.contains("rate limit exceeded")
            || message.contains("higher rate limit")
            || message.contains("#rate-limiting")
    }
}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl From<FromUtf8Error> for RequestError {
    fn from(value: FromUtf8Error) -> Self {
        Self(value.to_string())
    }
}

impl From<(StatusCode, Bytes)> for RequestError {
    fn from((status, bytes): (StatusCode, Bytes)) -> Self {
        Self(format!(
            "{} {} - {}",
            status.as_u16(),
            status.canonical_reason().unwrap(),
            String::from_utf8_lossy(&bytes)
        ))
    }
}

impl From<reqwest::Error> for RequestError {
    fn from(value: reqwest::Error) -> Self {
        Self(value.to_string())
    }
}

impl From<serde_json::Error> for RequestError {
    fn from(value: serde_json::Error) -> Self {
        Self(value.to_string())
    }
}
