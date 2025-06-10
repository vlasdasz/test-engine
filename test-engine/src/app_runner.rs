use std::{any::type_name, path::PathBuf, sync::Mutex, time::Duration};

use anyhow::Result;
use dispatch::{from_main, invoke_dispatched};
use gm::{
    LossyConvert, Platform,
    flat::{Point, Size},
};
use level::{LevelBase, LevelManager};
use log::debug;
use refs::{MainLock, Own, Rglica};
use tokio::time::sleep;
use ui::{Touch, TouchEvent, UIEvents, UIManager, View, ViewData, ViewFrame, ViewSubviews};
use vents::OnceEvent;
use wgpu::RenderPass;
use window::{ElementState, MouseButton, Screenshot, Window};
use winit::{
    event::{KeyEvent, TouchPhase},
    keyboard::Key,
};

use crate::{
    assets::Assets,
    level_drawer::LevelDrawer,
    ui::{Input, UI},
};

static WINDOW_READY: Mutex<OnceEvent> = Mutex::new(OnceEvent::const_default());
static CURSOR_POSITION: MainLock<Point> = MainLock::new();

pub struct AppRunner {
    window: Rglica<Window>,

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

        debug!("Logs setup");
    }

    fn new(first_view: Own<dyn View>) -> Self {
        #[cfg(desktop)]
        Assets::init(store::Paths::git_root().expect("git_root()"));
        #[cfg(mobile)]
        Assets::init(std::path::PathBuf::default());

        Self {
            cursor_position: Point::default(),
            first_view:      first_view.into(),
            window:          Rglica::default(),
        }
    }

    #[cfg(not(target_os = "android"))]
    pub async fn start(root_view: Own<dyn View>) -> Result<()> {
        Self::setup_log();
        #[cfg(feature = "debug")]
        Self::setup_debug_server().await?;
        Window::start(Self::new(root_view)).await
    }

    #[cfg(feature = "debug")]
    async fn setup_debug_server() -> Result<()> {
        use crate::debug_server::{
            on_debug_client_message, send_to_debug_client, start_listtening_for_debug_client,
        };

        start_listtening_for_debug_client().await;

        on_debug_client_message(|mut msg| {
            dbg!("recovka:");
            dbg!(&msg);

            msg.id += 555;

            send_to_debug_client(msg);
        })
        .await;

        Ok(())
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

    #[cfg(not(target_os = "android"))]
    pub async fn start_with_actor(
        first_view: Own<dyn View>,
        actions: impl std::future::Future<Output = Result<()>> + Send + 'static,
    ) -> Result<()> {
        Self::setup_log();

        let app = Self::new(first_view);

        tokio::spawn(async move {
            let recv = from_main(|| WINDOW_READY.lock().unwrap().val_async()).await;
            recv.await.unwrap();
            let _ = actions.await;
        });

        Window::start(app).await
    }

    pub fn set_window_title(title: impl Into<String>) {
        Window::set_title(title);
    }

    pub async fn set_window_size(size: impl Into<Size<u32>> + Send + 'static) {
        from_main(|| {
            Window::current().set_size(size);
        })
        .await;
        sleep(Duration::from_secs_f32(0.1)).await;
    }

    pub async fn take_screenshot() -> Result<Screenshot> {
        let recv = from_main(|| Window::current().request_screenshot()).await;
        let screenshot = recv.await?;
        Ok(screenshot)
    }

    pub fn fps() -> f32 {
        Window::current().fps()
    }
}

impl window::WindowEvents for AppRunner {
    fn window_ready(&mut self) {
        let view = UIManager::root_view_weak().__add_subview_internal(self.first_view.take().unwrap(), true);
        view.place().back();
        self.update();
        *LevelManager::update_interval() = 1.0 / Window::display_refresh_rate().lossy_convert();
        WINDOW_READY.lock().unwrap().trigger(());
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

    fn resize(&mut self, inner_position: Point, size: Size) {
        UIManager::root_view_weak().set_size(size.width, size.height);

        if Platform::IOS {
            UIManager::root_view_weak().set_position(inner_position);
        }

        UIEvents::size_changed().trigger(size);
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

    fn set_window(&mut self, app: Rglica<Window>) {
        self.window = app;
    }

    fn dropped_file(&mut self, path: PathBuf) {
        dbg!(type_name::<LevelBase>());
        UIManager::trigger_drop_file(path);
    }
}
