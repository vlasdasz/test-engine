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
    pub fn request(&self, url: impl ToString) -> Request {
        Request::make(format!("{}/{}", self.url, url.to_string()))
    }
}
