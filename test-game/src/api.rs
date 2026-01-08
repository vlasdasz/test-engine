use netrun::rest::Request;
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub(crate) struct User {
    pub id:   u32,
    pub name: String,
}

pub(crate) static TEST_REST_REQUEST: Request<(), Vec<User>> = Request::new("users");
