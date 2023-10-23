#![allow(incomplete_features)]
#![feature(specialization)]
#![feature(trait_upcasting)]
#![feature(arbitrary_self_types)]
#![feature(const_trait_impl)]

mod benchmark;
mod test_game;

use std::process::ExitCode;

use test_engine::{App, MakeApp};

use crate::test_game::TestApp;

#[tokio::main]
async fn main() -> ExitCode {
    TestApp::make_app().launch()
}
