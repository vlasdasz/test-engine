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

impl ToString for Error {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

pub type NetResult<T> = Result<T, Error>;
