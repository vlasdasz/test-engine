use reqwest::get;

use crate::Method;

// type Result<T = ()> = std::result::Result<T, String>;

pub struct Request {
    _method: Method,
    url:     String,
}

impl Request {
    pub fn make(url: impl ToString) -> Self {
        Self {
            _method: Method::Get,
            url:     url.to_string(),
        }
    }
}

impl Request {
    pub async fn call(&self) -> reqwest::Result<String> {
        get(&self.url).await?.text().await
    }
}
