use serde::{de::DeserializeOwned, Serialize};

use crate::Request;

pub struct API {
    url: String,
}

impl API {
    pub fn new(url: impl ToString) -> Self {
        Self { url: url.to_string() }
    }
}

impl API {
    pub fn request<Result: DeserializeOwned, Param: Serialize>(
        &self,
        url: impl ToString,
    ) -> Request<Result, Param> {
        Request::make(format!("http://{}/{}", self.url, url.to_string()))
    }

    // pub fn goto_moto<T: Deserialize>(&self, url: impl ToString) -> T {
    //     todo!()
    // }
}
