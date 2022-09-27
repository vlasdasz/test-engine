use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Error(String);

impl From<String> for Error {
    fn from(string: String) -> Self {
        Error(string)
    }
}

impl From<&'static str> for Error {
    fn from(str: &'static str) -> Self {
        Error(str.into())
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        format!("Network error: {}", error).into()
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        format!("Serialization error: {}", err).into()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Network error: {}", self.0)
    }
}

pub type NetResult<T> = Result<T, Error>;
