use std::fmt::Debug;

use reqwest::StatusCode;

#[derive(Debug)]
pub(crate) struct Response {
    #[allow(dead_code)]
    pub(crate) status: StatusCode,
    pub(crate) body:   String,
}
