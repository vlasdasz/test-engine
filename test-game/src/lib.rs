#![allow(incomplete_features)]
#![feature(specialization)]
#![feature(arbitrary_self_types)]
#![feature(const_trait_impl)]

mod interface;
mod levels;
mod no_physics;

pub use test_engine;

#[no_mangle]
pub extern "C" fn start_test_game() -> std::ffi::c_int {
    use winit::{
        application::ApplicationHandler,
        event::WindowEvent,
        event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
        window::WindowId,
    };

    struct App;

    impl ApplicationHandler for App {
        fn resumed(&mut self, event_loop: &ActiveEventLoop) {}
        fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {}
    }

    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Wait);

    event_loop.run_app(&mut App).unwrap();
    0
}

#[cfg(target_os = "android")]
pub fn start_test_game(app: test_engine::AndroidApp) {
    use test_engine::ui::ViewSetup;
    dbg!("HELLOOOddO");
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        test_engine::refs::set_current_thread_as_main();
        dbg!(test_engine::App::start(TestGameView::new(), app).await).unwrap()
    });
}

use test_engine::ui::Container;
#[cfg(target_os = "android")]
pub use test_engine::AndroidApp;
