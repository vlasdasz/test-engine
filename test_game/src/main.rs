#![allow(incomplete_features)]
#![feature(specialization)]
#![feature(arbitrary_self_types)]
#![feature(const_trait_impl)]

mod interface;

use anyhow::Result;
use test_engine::{ui::ViewSetup, App};

use crate::interface::color_view::ColorView;

// use crate::interface::test_game_view::TestGameView;

#[tokio::main]
async fn main() -> Result<()> {
    App::start(ColorView::new(), 1200, 1200).await
}
