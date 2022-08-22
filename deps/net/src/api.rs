use serde::{de::DeserializeOwned, Serialize};

use crate::{
    dispatch_request::{GetRequest, PostRequest},
    DispatchRequest,
};

pub struct API {
    base: &'static str,
}

impl API {
    pub const fn new(base: &'static str) -> Self {
        Self { base }
    }
}

impl API {
    pub const fn get<Result: DeserializeOwned>(&self, url: &'static str) -> GetRequest<Result> {
        GetRequest::make(self.base, url)
    }

    pub const fn post<Param: Serialize>(&self, url: &'static str) -> PostRequest<Param> {
        PostRequest::make(self.base, url)
    }

    pub const fn fetch<Param: Serialize, Result: DeserializeOwned>(
        &self,
        url: &'static str,
    ) -> DispatchRequest<Param, Result> {
        DispatchRequest::make(self.base, url)
    }
}
