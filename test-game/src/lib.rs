#![allow(incomplete_features)]
#![feature(specialization)]
#![feature(arbitrary_self_types)]
#![feature(const_trait_impl)]

mod interface;
mod levels;
mod test_game;

#[cfg(target_os = "ios")]
#[no_mangle]
extern "C" fn test_game() -> std::ffi::c_int {
    test_game::start_test_game();
    0
}

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: test_engine::AndroidApp) {
    test_game::start_test_game(app);
}

// #[cfg(target_os = "android")]
// pub fn start_android(event_loop: test_engine::EventLoop) {
//     use test_engine::ui::ViewSetup;
//     let runtime = tokio::runtime::Runtime::new().unwrap();
//     runtime.block_on(async {
//         test_engine::refs::set_current_thread_as_main();
//         App::start(TestGameView::new(), event_loop).await.unwrap()
//     });
// }
