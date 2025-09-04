use std::{
    any::type_name,
    path::PathBuf,
    sync::{Mutex, Once},
};

use anyhow::Result;
use dispatch::{from_main, invoke_dispatched, wait_for_next_frame};
use gm::{
    LossyConvert,
    flat::{Point, Size},
};
use level::{LevelBase, LevelManager};
use log::debug;
use refs::{Own, main_lock::MainLock};
use ui::{Container, Touch, TouchEvent, UIEvents, UIManager, View, ViewData};
use vents::OnceEvent;
use wgpu::RenderPass;
use window::{ElementState, MouseButton, Screenshot, Window};
use winit::{
    event::{KeyEvent, TouchPhase},
    keyboard::Key,
};

use crate::{
    App,
    app_starter::test_engine_start_with_app,
    assets::Assets,
    level_drawer::LevelDrawer,
    ui::{Input, UI},
};

static WINDOW_READY: Mutex<OnceEvent> = Mutex::new(OnceEvent::const_default());
static CURSOR_POSITION: MainLock<Point> = MainLock::new();

pub struct AppRunner {
    pub(crate) first_view: Option<Own<dyn View>>,
    pub cursor_position:   Point,
}

impl AppRunner {
    pub fn stop() {
        Window::close();
    }

    pub fn cursor_position() -> Point {
        *CURSOR_POSITION
    }

    #[cfg(not(android))]
    fn setup_log() {
        #[cfg(not_wasm)]
        {
            use fern::Dispatch;
            use log::{Level, LevelFilter};

            Dispatch::new()
                .level(LevelFilter::Warn)
                .level_for("test_engine", LevelFilter::Debug)
                .level_for("shopping", LevelFilter::Debug)
                .format(|out, message, record| {
                    let level_icon = match record.level() {
                        Level::Error => "🔴",
                        Level::Warn => "🟡",
                        Level::Info => "🟢",
                        Level::Debug => "🔵",
                        Level::Trace => "⚪",
                    };

                    let location = false;
                    let module = false;

                    let mut log = format!("{level_icon} {message}");

                    if location {
                        log = format!(
                            "[{}::{}] {}",
                            record.file().unwrap_or_default(),
                            record.line().unwrap_or_default(),
                            log
                        );
                    }

                    if module {
                        log = format!("{} {}", record.module_path().unwrap_or_default(), log);
                    }

                    out.finish(format_args!("{log}"));
                })
                .chain(std::io::stdout())
                .apply()
                .expect("Failed to initialize logging");
        }

        debug!("Logs setup");
    }

    pub fn new(first_view: Own<dyn View>) -> Self {
        #[cfg(desktop)]
        Assets::init(store::Paths::git_root().expect("git_root()"));
        #[cfg(mobile)]
        Assets::init(std::path::PathBuf::default());

        Self {
            cursor_position: Point::default(),
            first_view:      first_view.into(),
        }
    }

    #[cfg(target_os = "android")]
    pub(crate) async fn start(first_view: Own<dyn View>, app: crate::AndroidApp) -> Result<()> {
        dbg!("PENIJEE");

        std::panic::set_hook(Box::new(|pan| {
            let backtrace = std::backtrace::Backtrace::force_capture();
            println!("Custom panic hook");
            dbg!(&pan);
            dbg!(&pan.payload_as_str());
            dbg!(&backtrace);
            eprintln!("Backtrace: {}", backtrace);
        }));

        dbg!("Panic hook set");

        use winit::platform::android::EventLoopBuilderExtAndroid;

        // android_logger::try_init(android_logger::Config::default().
        // with_max_level(LevelFilter::Trace));

        // try_init();

        android_logger::init_once(android_logger::Config::default().with_max_level(log::LevelFilter::Warn));

        log::error!("AAAASOOOOOO");

        let event_loop: crate::EventLoop =
            crate::EventLoop::with_user_event().with_android_app(app).build().unwrap();

        log::error!("EVANTO");

        Window::start(Self::new(first_view), event_loop).await
    }

    #[cfg(not_wasm)]
    pub fn start_with_actor(
        actions: impl std::future::Future<Output = Result<()>> + Send + 'static,
    ) -> Result<()> {
        use ui::Setup;

        struct ActorApp;

        impl App for ActorApp {
            fn new() -> Self
            where Self: Sized {
                ActorApp {}
            }

            fn make_root_view(&self) -> Own<dyn View> {
                Container::new()
            }
        }

        Self::setup_log();

        std::thread::spawn(move || {
            WINDOW_READY.lock().unwrap().sub(|| {
                async_std::task::block_on(actions).unwrap();
            });
        });

        test_engine_start_with_app(Box::new(ActorApp::new()));

        Ok(())
    }

    pub fn set_window_title(title: impl Into<String>) {
        Window::set_title(title);
    }

    pub fn set_window_size(size: impl Into<Size<u32>> + Send + 'static) {
        from_main(|| {
            Window::current().set_size(size);
        });
        wait_for_next_frame();
    }

    pub fn take_screenshot() -> Result<Screenshot> {
        let recv = from_main(|| Window::current().request_screenshot());
        let screenshot = recv.recv()?;
        Ok(screenshot)
    }

    pub fn fps() -> f32 {
        Window::current().fps()
    }
}

impl window::WindowEvents for AppRunner {
    fn window_ready(&mut self) {
        static INIT: Once = Once::new();

        INIT.call_once(|| {
            debug!("window ready");
            let mut root = UIManager::root_view();
            let view = root.add_subview_to_root(self.first_view.take().unwrap());
            view.place().back();

            UIManager::on_scale_changed(root, move |scale| {
                root.rescale_root(scale);
            });

            self.update();
            *LevelManager::update_interval() = 1.0 / Window::display_refresh_rate().lossy_convert();

            #[cfg(not_wasm)]
            std::thread::spawn(|| {
                WINDOW_READY.lock().unwrap().trigger(());
            });
        });
    }

    fn update(&mut self) {
        UIManager::free_deleted_views();
        invoke_dispatched();
        LevelDrawer::update();
        UI::update();
    }

    fn render<'a>(&'a mut self, pass: &mut RenderPass<'a>) {
        if UIManager::window_resolution().has_no_area() {
            return;
        }

        LevelDrawer::draw(pass);
        UI::draw(pass);
    }

    fn resize(&mut self, inner_pos: Point, outer_pos: Point, inner_size: Size, outer_size: Size) {
        UIManager::set_scale(UIManager::display_scale());
        LevelManager::set_scale(UIManager::display_scale());
        UIManager::root_view().resize_root(inner_pos, outer_pos, inner_size, outer_size, UIManager::scale());
        UIEvents::size_changed().trigger(());
        self.update();
    }

    fn mouse_moved(&mut self, position: Point) -> bool {
        self.cursor_position = position;
        *CURSOR_POSITION.get_mut() = position;
        Input::process_touch_event(Touch {
            id: 1,
            position,
            event: TouchEvent::Moved,
            button: MouseButton::Left,
        })
    }

    fn mouse_event(&mut self, state: ElementState, button: MouseButton) -> bool {
        Input::process_touch_event(Touch {
            id: 1,
            position: self.cursor_position,
            event: state.into(),
            button,
        })
    }

    fn mouse_scroll(&mut self, delta: Point) {
        UIManager::trigger_scroll(delta);
    }

    fn touch_event(&mut self, touch: winit::event::Touch) -> bool {
        Input::process_touch_event(Touch {
            id:       1,
            position: (touch.location.x, touch.location.y).into(),
            event:    match touch.phase {
                TouchPhase::Started => TouchEvent::Began,
                TouchPhase::Moved => TouchEvent::Moved,
                TouchPhase::Ended | TouchPhase::Cancelled => TouchEvent::Ended,
            },
            button:   MouseButton::Left,
        })
    }

    fn key_event(&mut self, event: KeyEvent) {
        if !event.state.is_pressed() {
            return;
        }

        if let Key::Named(key) = event.logical_key {
            Input::on_key(key);
        }

        if let Some(ch) = event.logical_key.to_text() {
            Input::on_char(ch.chars().last().unwrap());
        }
    }

    fn dropped_file(&mut self, path: PathBuf) {
        dbg!(type_name::<LevelBase>());
        UIManager::trigger_drop_file(path);
    }
}
