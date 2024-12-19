#![allow(incomplete_features)]
#![feature(specialization)]
#![feature(arbitrary_self_types)]
#![feature(const_trait_impl)]

mod interface;
mod levels;
mod no_physics;

pub use test_engine;

use crate::interface::test_game_view::TestGameView;

#[cfg(not(target_os = "android"))]
#[unsafe(no_mangle)]
pub extern "C" fn start_test_game() -> std::ffi::c_int {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        #[cfg(mobile)]
        test_engine::refs::set_current_thread_as_main();
        test_engine::App::start::<TestGameView>().await.unwrap();
    });
    0
}

#[cfg(target_os = "android")]
pub fn start_test_game(app: test_engine::AndroidApp) {
    use test_engine::ui::Setup;
    dbg!("HELLOOOddO");
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        test_engine::refs::set_current_thread_as_main();
        dbg!(test_engine::App::start(TestGameView::new(), app).await).unwrap()
    });
}

#[cfg(target_os = "android")]
pub use test_engine::AndroidApp;
