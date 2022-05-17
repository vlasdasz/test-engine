use std::marker::PhantomData;
use reqwest::get;
use serde::de::DeserializeOwned;
use serde_json::from_str;

use crate::Method;

// type Result<T = ()> = std::result::Result<T, String>;

pub struct Request<Result: DeserializeOwned> {
    _method: Method,
    url:     String,
    _a: PhantomData<Result>,
}

impl<Result: DeserializeOwned> Request<Result> {
    pub fn make(url: impl ToString) -> Self {
        Self {
            _method: Method::Get,
            url:     url.to_string(),
            _a: Default::default()
        }
    }

    async fn call(&self) -> reqwest::Result<String> {
        get(&self.url).await?.text().await
    }

    pub async fn gotome(&self) -> reqwest::Result<Result> {
        let string = self.call().await?;
        let v: Result = from_str(&string).unwrap();
        Ok(v)
    }
}
