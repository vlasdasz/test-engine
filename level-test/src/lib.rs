#![allow(incomplete_features)]
#![feature(specialization)]
#![feature(arbitrary_self_types)]
#![feature(const_trait_impl)]

mod interface;
mod levels;

use crate::interface::level_test_view::LevelTestView;

#[cfg(not(target_os = "android"))]
pub fn start_level_test() {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        #[cfg(mobile)]
        test_engine::refs::set_current_thread_as_main();
        test_engine::App::start::<LevelTestView>().await.unwrap();
    });
}

#[cfg(target_os = "android")]
pub fn start_test_game(app: test_engine::AndroidApp) {
    use test_engine::ui::Setup;
    dbg!("HELLOOOddO");
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        test_engine::refs::set_current_thread_as_main();
        dbg!(test_engine::App::start(LevelTestView::new(), app).await).unwrap()
    });
}

#[cfg(target_os = "android")]
pub use test_engine::AndroidApp;

#[cfg(target_os = "ios")]
#[unsafe(no_mangle)]
extern "C" fn test_game() -> std::ffi::c_int {
    start_test_game();
    0
}
