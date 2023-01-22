use std::{borrow::Borrow, fmt::Debug, marker::PhantomData};

use log::debug;
use reqwest::{Client, RequestBuilder};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{from_str, to_string};

use crate::{response::Response, Method, NetResult, API};

pub struct Req<Param, Result> {
    url:    &'static str,
    method: Method,
    _data:  PhantomData<(*const Param, *const Result)>,
}

pub type GetReq<T> = Req<(), T>;

unsafe impl<T, U> Send for Req<T, U> {}
unsafe impl<T, U> Sync for Req<T, U> {}

impl<R, P> Req<R, P> {
    pub const fn make(url: &'static str) -> Self {
        Self {
            url,
            method: Method::Get,
            _data: PhantomData,
        }
    }

    fn full_url(&self) -> String {
        format!("http://{}/{}", API::base_url(), self.url)
    }
}

impl<R, P> const From<&'static str> for Req<R, P> {
    fn from(url: &'static str) -> Self {
        Self::make(url)
    }
}

impl<Output: DeserializeOwned> Req<(), Output> {
    pub async fn get(self) -> NetResult<Output> {
        request_object(&self.method, self.full_url(), None).await
    }
}

impl<Param: Serialize> Req<Param, ()> {
    pub async fn post(&self, param: impl Borrow<Param>) -> NetResult<()> {
        let body = to_string(param.borrow())?;
        request(&self.method, self.full_url(), body.into()).await?;
        Ok(())
    }
}

impl<Param: Serialize, Output: DeserializeOwned + Debug> Req<Param, Output> {
    pub async fn fetch(&self, param: impl Borrow<Param>) -> NetResult<Output> {
        let body = to_string(param.borrow())?;
        request_object(&self.method, self.full_url(), body.into()).await
    }
}

async fn request(method: &Method, url: String, body: Option<String>) -> NetResult<Response> {
    let url = url.to_string();
    let client = Client::new();

    let req = match method {
        Method::Get => client.get(&url),
        Method::Post => client.post(&url),
    };

    let req = add_headers(req);

    let req = match &body {
        Some(body) => req.body(body.clone()),
        None => req,
    };

    debug!("Request - {url} - {method} {body:?}");

    let res = req.send().await?;

    let status = res.status();
    let body = res.text().await?;

    let response = Response { status, body };

    debug!("Response - {response:?}");

    Ok(response)
}

async fn request_object<T>(method: &Method, url: String, body: Option<String>) -> NetResult<T>
where T: DeserializeOwned {
    let response = request(method, url, body).await?;

    if response.status == 500 {
        Err(from_str(&response.body)?)
    } else {
        Ok(from_str(&response.body)?)
    }
}

fn add_headers(request: RequestBuilder) -> RequestBuilder {
    let mut request = request;
    for (key, value) in API::headers() {
        request = request.header(key, value)
    }
    request
}
