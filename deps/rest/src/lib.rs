#![feature(const_default_impls)]

mod api;
mod errors;
mod method;
mod request;
mod response;

pub use api::API;
pub use errors::{Error, NetResult};
pub use method::Method;
pub use request::*;
