use std::collections::HashMap;

use rtools::static_init;

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
    pub fn use_json(mut self) -> Self {
        self.headers.insert("content-type".to_string(), "application/json".to_string());
        self
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

    pub fn remove_header(key: impl ToString) {
        Self::get().headers.remove(&key.to_string());
    }

    pub fn add_header(key: impl ToString, value: impl ToString) {
        Self::get().headers.insert(key.to_string(), value.to_string());
    }
}
