use std::{
    collections::HashMap,
    sync::{Mutex, OnceLock},
};

static STATIC_API: OnceLock<API> = OnceLock::new();

#[derive(Debug)]
pub struct API {
    base_url: &'static str,
    headers:  Mutex<HashMap<String, String>>,
}

impl API {
    pub fn init(base_url: &'static str) {
        STATIC_API
            .set(Self {
                base_url,
                headers: Mutex::new(HashMap::from([(
                    "content-type".to_string(),
                    "application/json".to_string(),
                )])),
            })
            .unwrap();
    }

    pub fn is_ok() -> bool {
        STATIC_API.get().is_some()
    }

    fn get() -> &'static Self {
        STATIC_API.get().unwrap()
    }
}

impl API {
    pub fn base_url() -> &'static str {
        Self::get().base_url
    }

    pub fn headers() -> HashMap<String, String> {
        Self::get().headers.lock().unwrap().clone()
    }

    pub fn remove_header(key: impl ToString) {
        Self::get().headers.lock().unwrap().remove(&key.to_string());
    }

    pub fn add_header(key: impl ToString, value: impl ToString) {
        Self::get().headers.lock().unwrap().insert(key.to_string(), value.to_string());
    }
}
