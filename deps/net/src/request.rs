use std::borrow::Borrow;

use reqwest::{Client, RequestBuilder};
use rtools::Rglica;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{from_str, to_string};
use tao_log::{debugv, trace, tracev};

use crate::{Method, NetResult, API};

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

    fn full_url(&self) -> String {
        format!("http://{}/{}", API::base_url(), self.url)
    }
}

impl<Result: DeserializeOwned> Request<(), Result> {
    async fn call(&self) -> reqwest::Result<String> {
        let client = Client::new();
        let get = client.get(self.full_url());
        let get = add_headers(get);
        get.send().await?.text().await
    }

    pub async fn get(&self) -> NetResult<Result> {
        Ok(from_str(&self.call().await?)?)
    }
}

impl<Param: Serialize> Request<Param, ()> {
    pub async fn post(&self, param: impl Borrow<Param>) -> NetResult<()> {
        let body = to_string(param.borrow())?;
        tracev!(&body);
        trace!("Body: {}", body);
        let client = Client::new();
        let post = client.post(&self.full_url());
        let post = add_headers(post);
        post.body(body).send().await?;
        Ok(())
    }
}

impl<Param: Serialize, Output: DeserializeOwned> Request<Param, Output> {
    pub async fn fetch(&self, param: impl Borrow<Param>) -> NetResult<Output> {
        let body = to_string(param.borrow()).unwrap();
        tracev!(&body);
        trace!("Body: {}", body);
        let client = Client::new();
        trace!("Full url: {}", self.full_url());
        let post = client.post(&self.full_url());
        let post = add_headers(post);
        let body_string = post.body(body).send().await?.text().await?;
        Ok(from_str(debugv!(&body_string))?)
    }
}

fn add_headers(request: RequestBuilder) -> RequestBuilder {
    let mut request = request;
    for (key, value) in API::headers() {
        request = request.header(key, value)
    }
    request
}
