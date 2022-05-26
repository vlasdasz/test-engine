use std::borrow::Borrow;

use reqwest::{get, Client};
use rtools::Rglica;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{from_str, to_string};

use crate::Method;

pub struct Request<Param, Result> {
    url:     &'static str,
    _method: Method,
    _data:   Rglica<(*const Param, *const Result)>,
}

impl<R, P> Request<R, P> {
    pub const fn make(url: &'static str) -> Self {
        Self {
            url,
            _method: Method::Get,
            _data: Rglica::const_default(),
        }
    }
}

impl<Result: DeserializeOwned> Request<(), Result> {
    async fn call(&self) -> reqwest::Result<String> {
        get(self.url).await?.text().await
    }

    pub async fn get(&'static self) -> reqwest::Result<Result> {
        let string = self.call().await?;
        let v: Result = from_str(&string).unwrap();
        Ok(v)
    }
}

impl<Param: Serialize> Request<Param, ()> {
    pub async fn post(&self, param: impl Borrow<Param>) -> reqwest::Result<()> {
        let string = to_string(param.borrow()).unwrap();
        let client = Client::new();
        client.post(self.url).body(string).send().await?;
        Ok(())
    }
}

impl<Param: Serialize, Result: DeserializeOwned> Request<Result, Param> {
    pub async fn fetch(&self, param: impl AsRef<Param>) -> reqwest::Result<Result> {
        let string = to_string(param.as_ref()).unwrap();
        let client = Client::new();
        let text = client.post(self.url).body(string).send().await?.text().await?;
        Ok(from_str(&text).unwrap())
    }
}
