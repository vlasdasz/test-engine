use serde::de::DeserializeOwned;

use crate::DispatchRequest;

pub struct API {
    base: &'static str,
}

impl API {
    pub const fn new(base: &'static str) -> Self {
        Self { base }
    }
}

impl API {
    pub const fn get<Result: DeserializeOwned>(&self, url: &'static str) -> DispatchRequest<(), Result> {
        DispatchRequest::make(self.base, url)
    }
}
