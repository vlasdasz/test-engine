use std::borrow::Borrow;

use reqwest::{get, Client};
use rtools::Rglica;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{from_str, to_string};

use crate::{Method, NetResult};

pub struct Request<Param, Result> {
    base:    &'static str,
    url:     &'static str,
    _method: Method,
    _data:   Rglica<(*const Param, *const Result)>,
}

impl<R, P> Request<R, P> {
    pub const fn make(base: &'static str, url: &'static str) -> Self {
        Self {
            base,
            url,
            _method: Method::Get,
            _data: Rglica::const_default(),
        }
    }

    fn full_url(&self) -> String {
        format!("http://{}/{}", self.base, self.url)
    }
}

impl<Result: DeserializeOwned> Request<(), Result> {
    async fn call(&self) -> reqwest::Result<String> {
        get(&self.full_url()).await?.text().await
    }

    pub async fn get(&self) -> NetResult<Result> {
        Ok(from_str(&self.call().await?)?)
    }
}

impl<Param: Serialize> Request<Param, ()> {
    pub async fn post(&self, param: impl Borrow<Param>) -> NetResult<()> {
        let string = to_string(param.borrow())?;
        let client = Client::new();
        client.post(&self.full_url()).body(string).send().await?;
        Ok(())
    }
}

impl<Param: Serialize, Output: DeserializeOwned> Request<Param, Output> {
    pub async fn fetch(&self, param: impl Borrow<Param>) -> NetResult<Output> {
        let string = to_string(param.borrow()).unwrap();
        let client = Client::new();
        let text = client
            .post(&self.full_url())
            .body(string)
            .send()
            .await?
            .text()
            .await?;
        Ok(from_str(&text)?)
    }
}
