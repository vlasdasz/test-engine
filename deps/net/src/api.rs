use serde::de::DeserializeOwned;

use crate::dispath_request::GetRequest;

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
}
