// use tokio::runtime::Runtime;

use log::error;
use window::{AppHandler, Window};
use winit::event_loop::{ControlFlow, EventLoop};

use crate::{AppRunner, app::test_engine_create_app};

#[cfg(target_arch = "wasm32")]
fn run_app(event_loop: EventLoop<Window>, app: &'static mut AppHandler) {
    // Sets up panics to go to the console.error in browser environments
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Error).expect("Couldn't initialize logger");

    // Runs the app async via the browsers event loop
    use winit::platform::web::EventLoopExtWebSys;
    wasm_bindgen_futures::spawn_local(async move {
        event_loop.spawn_app(app);
    });
}

#[cfg(not(target_arch = "wasm32"))]
fn run_app(event_loop: EventLoop<Window>, mut app: &mut AppHandler) {
    let _ = event_loop.run_app(app);
}

#[cfg(not(target_os = "android"))]
#[unsafe(no_mangle)]
pub extern "C" fn test_engine_start_app() -> std::ffi::c_int {
    dbg!("test_engine_start_app");

    #[cfg(mobile)]
    crate::refs::set_current_thread_as_main();

    let event_loop = EventLoop::<Window>::with_user_event().build().unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);

    #[allow(unused_unsafe)]
    let app = unsafe { test_engine_create_app() };
    app.setup();

    let app = AppHandler::new(
        app.initial_size(),
        AppRunner::new(app.make_root_view()),
        &event_loop,
    );

    run_app(event_loop, app);

    0
}

#[cfg(not(target_os = "android"))]
#[unsafe(no_mangle)]
pub extern "C" fn test_engine_start_app_old() -> std::ffi::c_int {
    dbg!("aa");

    // Sets up panics to go to the console.error in browser environments
    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init_with_level(log::Level::Trace).expect("Couldn't initialize logger");
    }

    // let runtime = Runtime::new().unwrap();
    // runtime.block_on(async {
    #[cfg(mobile)]
    crate::refs::set_current_thread_as_main();

    #[allow(unused_unsafe)]
    let app = unsafe { test_engine_create_app() };
    app.setup();

    // crate::AppRunner::start(app.initial_size(), app.make_root_view()).unwrap();
    // });
    0
}

#[cfg(target_os = "android")]
pub fn test_engine_start_app(android_app: crate::AndroidApp) {
    dbg!("HELLOOOddO");
    let runtime = Runtime::new().unwrap();
    runtime.block_on(async {
        crate::refs::set_current_thread_as_main();

        let app = unsafe { test_engine_create_app() };
        app.setup();

        dbg!(crate::AppRunner::start(app.make_root_view(), android_app).await).unwrap()
    });
}
