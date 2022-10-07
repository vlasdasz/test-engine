#![allow(incomplete_features)]
#![allow(clippy::module_inception)]
#![feature(specialization)]
#![feature(trait_upcasting)]

use test_engine::{paths::home, rtools::init_log, Screen};
use ui::refs::{Own};

use crate::{
    benchmark::UIDebugView,
};

mod benchmark;
mod test_game;
mod ui_test;

#[tokio::main]
async fn main() {
    init_log(false, 4);

    let mut screen = Screen::new(
        (1000, 600),
        &home().join("test_engine"),
        Own::<UIDebugView>::default(),
    );

    // screen.ui.set_level(Strong::<TestGameLevel>::default());

    screen.start_main_loop();
}
