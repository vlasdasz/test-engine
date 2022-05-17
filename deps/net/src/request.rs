use reqwest::get;
use serde::de::DeserializeOwned;
use serde_json::from_str;

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

    pub async fn gotome<T: DeserializeOwned>(&self) -> reqwest::Result<T> {
        let string = self.call().await?;
        let v: T = from_str(&string).unwrap();
        Ok(v)
    }
}
