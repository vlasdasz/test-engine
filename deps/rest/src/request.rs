use std::{any::type_name, borrow::Borrow, collections::HashMap, fmt::Debug, marker::PhantomData};

use log::{debug, error};
use reqwest::Client;
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
        format!("{}/{}", API::base_url(), self.url)
    }
}

#[const_trait]
pub trait ToReq<R, P, T> {
    fn req(self) -> Req<R, P>;
}

impl<R, P> const ToReq<R, P, &'static str> for &'static str {
    fn req(self) -> Req<R, P> {
        Req::<R, P>::make(self)
    }
}

impl<Output: DeserializeOwned> Req<(), Output> {
    pub async fn get(self) -> NetResult<Output> {
        request_object(self.method, self.full_url(), None).await
    }
}

impl<Param: Serialize> Req<Param, ()> {
    pub async fn post(&self, param: impl Borrow<Param>) -> NetResult<()> {
        let body = to_string(param.borrow())?;
        raw_request(self.method, self.full_url(), &API::headers(), body.into()).await?;
        Ok(())
    }
}

impl<Param: Serialize, Output: DeserializeOwned + Debug> Req<Param, Output> {
    pub async fn fetch(&self, param: impl Borrow<Param>) -> NetResult<Output> {
        let body = to_string(param.borrow())?;
        request_object(self.method, self.full_url(), body.into()).await
    }
}

async fn request_object<T>(method: Method, url: String, body: Option<String>) -> NetResult<T>
where T: DeserializeOwned {
    let response = raw_request(method, url, &API::headers(), body).await?;

    if response.status != 200 {
        error!("Object request failed: {response:?}");
        Err(parse(&response.body)?)
    } else {
        Ok(parse(&response.body)?)
    }
}

fn parse<T: DeserializeOwned>(json: impl ToString) -> NetResult<T> {
    let json = json.to_string();
    match from_str(&json) {
        Ok(obj) => Ok(obj),
        Err(error) => {
            let message = format!("Failed to parse {} from {json}. Error: {error}", type_name::<T>());
            error!("{message}");
            Err(message.into())
        }
    }
}

pub async fn raw_request(
    method: Method,
    url: impl ToString,
    headers: &HashMap<String, String>,
    body: Option<String>,
) -> NetResult<Response> {
    let url = url.to_string();
    let client = Client::new();

    let mut request = match method {
        Method::Get => client.get(&url),
        Method::Post => client.post(&url),
    };

    for (key, value) in headers {
        request = request.header(key, value)
    }

    let request = match &body {
        Some(body) => request.body(body.clone()),
        None => request,
    };

    debug!("Request - {url} - {method} {body:?}");

    let response = request.send().await.map_err(|e| {
        error!("Failed to send request: {e}");
        e
    })?;

    let status = response.status();
    let body = response.text().await?;

    let response = Response { url, status, body };

    debug!("Response - {response:?}");

    Ok(response)
}
