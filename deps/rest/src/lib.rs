#![feature(const_default_impls)]
#![feature(const_trait_impl)]

mod api;
mod errors;
mod method;
mod request;
mod response;

pub use api::API;
pub use errors::{NetResult, RestError, *};
pub use method::Method;
pub use request::*;
