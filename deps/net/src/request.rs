use reqwest::get;
use crate::Method;

// type Result<T = ()> = std::result::Result<T, String>;

pub struct Request {
    _method: Method,
    url: &'static str,
}

impl Request {
    pub fn make(url: &'static str) -> Self {
        Self { _method: Method::Get, url }
    }
}

impl Request {
    pub async fn call(&self) -> reqwest::Result<String> {
        get(self.url).await?.text().await
    }
}

