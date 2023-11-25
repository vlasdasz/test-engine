#![allow(incomplete_features)]
#![feature(specialization)]
#![feature(arbitrary_self_types)]
#![feature(const_trait_impl)]

mod benchmark;
mod test_game;

use anyhow::Result;
use test_engine::{App, MakeApp};

use crate::test_game::test_app::TestApp;

#[tokio::main]
async fn main() -> Result<()> {
    TestApp::make_app().launch()
}
