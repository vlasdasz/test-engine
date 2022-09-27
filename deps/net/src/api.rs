use std::collections::HashMap;

use rtools::static_init;
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    dispatch_request::{GetRequest, PostRequest},
    DispatchRequest,
};

#[derive(Default)]
pub struct API {
    base_url: String,
    headers:  HashMap<String, String>,
}
static_init!(API);

impl API {
    pub fn new(base_url: impl ToString) -> Self {
        Self {
            base_url: base_url.to_string(),
            headers:  Default::default(),
        }
    }
}

impl API {
    pub fn base_url() -> &'static str {
        &Self::get().base_url
    }

    pub fn set_base_url(url: impl ToString) {
        Self::get().base_url = url.to_string()
    }

    pub fn headers() -> &'static HashMap<String, String> {
        &Self::get().headers
    }

    pub fn clear_headers() {
        Self::get().headers.clear()
    }

    pub fn set_token(token: impl ToString) {
        Self::add_header("token", token)
    }

    pub fn add_header(key: impl ToString, value: impl ToString) {
        Self::get().headers.insert(key.to_string(), value.to_string());
    }
}

impl API {
    pub fn get_request<Result: DeserializeOwned>(url: &'static str) -> GetRequest<Result> {
        GetRequest::make(url)
    }

    pub fn post_request<Param: Serialize>(url: &'static str) -> PostRequest<Param> {
        PostRequest::make(url)
    }

    pub fn fetch_request<Param: Serialize, Result: DeserializeOwned>(
        url: &'static str,
    ) -> DispatchRequest<Param, Result> {
        DispatchRequest::make(url)
    }
}
