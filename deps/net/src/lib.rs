#![feature(const_default_impls)]

mod api;
mod dispatch_request;
mod errors;
mod method;
mod request;

pub use api::API;
pub use errors::{Error, NetResult};
pub use method::Method;
pub use request::*;
