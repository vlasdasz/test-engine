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
    fn from(_: reqwest::Error) -> Self {
        "Network error".into()
    }
}

impl From<serde_json::Error> for Error {
    fn from(_: serde_json::Error) -> Self {
        "Serialization error".into()
    }
}

pub type NetResult<T> = Result<T, Error>;
