use std::{borrow::Borrow, marker::PhantomData};

use log::trace;
use reqwest::{Client, RequestBuilder};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{from_str, to_string};

use crate::{Method, NetResult, API};

pub struct Req<Param, Result> {
    url:     &'static str,
    _method: Method,
    _data:   PhantomData<(*const Param, *const Result)>,
}

pub type GetReq<T> = Req<(), T>;

unsafe impl<T, U> Send for Req<T, U> {}
unsafe impl<T, U> Sync for Req<T, U> {}

impl<R, P> Req<R, P> {
    pub const fn make(url: &'static str) -> Self {
        Self {
            url,
            _method: Method::Get,
            _data: PhantomData,
        }
    }

    fn full_url(&self) -> String {
        format!("http://{}/{}", API::base_url(), self.url)
    }
}

impl<Param, Output> Req<Param, Output> {
    async fn call(&self) -> reqwest::Result<String> {
        let client = Client::new();
        let get = client.get(self.full_url());
        let get = add_headers(get);
        get.send().await?.text().await
    }
}

impl<Output: DeserializeOwned> Req<(), Output> {
    pub async fn get(self) -> NetResult<Output> {
        Ok(from_str(&self.call().await?)?)
    }
}

impl<Param: Serialize> Req<Param, ()> {
    pub async fn post(&self, param: impl Borrow<Param>) -> NetResult<()> {
        let body = to_string(param.borrow())?;
        trace!("Body: {}", body);
        let client = Client::new();
        let post = client.post(&self.full_url());
        let post = add_headers(post);
        post.body(body).send().await?;
        Ok(())
    }
}

impl<Param: Serialize, Output: DeserializeOwned> Req<Param, Output> {
    pub async fn fetch(&self, param: impl Borrow<Param>) -> NetResult<Output> {
        let body = to_string(param.borrow()).unwrap();
        trace!("Body: {}", body);
        let client = Client::new();
        trace!("Full url: {}", self.full_url());
        let post = client.post(&self.full_url());
        let post = add_headers(post);
        let body_string = post.body(body).send().await?.text().await?;
        Ok(from_str(&body_string)?)
    }
}

fn add_headers(request: RequestBuilder) -> RequestBuilder {
    let mut request = request;
    for (key, value) in API::headers() {
        request = request.header(key, value)
    }
    request
}
