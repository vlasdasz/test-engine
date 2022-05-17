use serde::de::DeserializeOwned;
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
    pub fn request<T: DeserializeOwned>(&self, url: impl ToString) -> Request<T> {
        Request::make(format!("http://{}/{}", self.url, url.to_string()))
    }

    // pub fn goto_moto<T: Deserialize>(&self, url: impl ToString) -> T {
    //     todo!()
    // }
}
