use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use actix_web::ResponseError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RestError {
    message: String,
}

impl Error for RestError {}

impl From<String> for RestError {
    fn from(message: String) -> Self {
        Self { message }
    }
}

impl From<&'static str> for RestError {
    fn from(str: &'static str) -> Self {
        Self { message: str.into() }
    }
}

impl From<reqwest::Error> for RestError {
    fn from(error: reqwest::Error) -> Self {
        format!("Network error: {error}").into()
    }
}

impl From<serde_json::Error> for RestError {
    fn from(err: serde_json::Error) -> Self {
        format!("Serialization error: {err}").into()
    }
}

impl Display for RestError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

pub type NetResult<T> = Result<T, RestError>;

impl ResponseError for RestError {}
