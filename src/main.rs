#![allow(incomplete_features)]
#![feature(specialization)]
#![feature(trait_upcasting)]
#![feature(arbitrary_self_types)]

use test_engine::{paths::home, rtools::init_log, Screen};
use ui::refs::{dump_ref_stats, enable_ref_stats_counter, Own};

use crate::benchmark::UIDebugView;

mod benchmark;
mod test_game;
mod ui_test;

#[tokio::main]
async fn main() {
    enable_ref_stats_counter(true);

    init_log(false, 4);

    let mut screen = Screen::new(
        (1000, 600),
        &home().join("test_engine"),
        Own::<UIDebugView>::default(),
    );

    // screen.ui.set_level(Strong::<TestGameLevel>::default());

    screen.start_main_loop();

    drop(screen);

    dump_ref_stats();
}
