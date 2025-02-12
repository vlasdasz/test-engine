use std::{any::type_name, io::Write, path::PathBuf, sync::Mutex, time::Duration};

use anyhow::Result;
use dispatch::{from_main, invoke_dispatched};
use env_logger::Builder;
use gm::{
    LossyConvert,
    flat::{Point, Size},
};
use level::{LevelBase, LevelManager};
use log::{Level, LevelFilter};
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

pub struct App {
    window: Rglica<Window>,

    pub(crate) first_view: Option<Own<dyn View>>,
    pub cursor_position:   Point,
}

impl App {
    pub fn stop() {
        Window::close();
    }

    pub fn cursor_position() -> Point {
        *CURSOR_POSITION
    }

    fn setup_log() {
        Builder::from_default_env()
            .filter_level(LevelFilter::Debug)
            .filter_module("winit::platform_impl::platform::app_state", LevelFilter::Error)
            .filter_module("winit::Window::request_redraw", LevelFilter::Error)
            .filter_module("wgpu_core::device", LevelFilter::Warn)
            .filter_module("wgpu_core::present", LevelFilter::Warn)
            .filter_module("wgpu_core::resource", LevelFilter::Warn)
            .filter_module("wgpu_core::instance", LevelFilter::Warn)
            .filter_module("wgpu_hal::metal::surface", LevelFilter::Warn)
            .filter_module("wgpu_hal::metal::device", LevelFilter::Warn)
            .filter_module("wgpu_hal::metal", LevelFilter::Warn)
            .filter_module("wgpu_hal::vulkan::adapter", LevelFilter::Warn)
            .filter_module("wgpu_hal::vulkan::instance", LevelFilter::Warn)
            .filter_module("wgpu_hal::dx12::instance", LevelFilter::Warn)
            .filter_module("wgpu_hal::gles::adapter", LevelFilter::Warn)
            .filter_module("wgpu_hal::gles::wgl", LevelFilter::Warn)
            .filter_module("wgpu_hal::gles::egl", LevelFilter::Warn)
            .filter_module("naga::front", LevelFilter::Warn)
            .filter_module("naga::proc::constant_evaluator", LevelFilter::Warn)
            .filter_module("naga::valid::interface", LevelFilter::Warn)
            .filter_module("naga::valid::function", LevelFilter::Warn)
            .filter_module("sqlx_core::logger", LevelFilter::Warn)
            .filter_module("hyper_util::client::legacy::pool", LevelFilter::Warn)
            .filter_module("hyper_util::client::legacy::connect::dns", LevelFilter::Warn)
            .filter_module("hyper_util::client::legacy::connect::http", LevelFilter::Warn)
            .format(|f, record| {
                let level = match record.level() {
                    Level::Error => "🔴",
                    Level::Warn => "🟡",
                    Level::Info => "🟢",
                    Level::Debug => "🔵",
                    Level::Trace => "⚪",
                };

                // let level = match record.level() {
                //     Level::Error => "ERROR",
                //     Level::Warn => "WARNING",
                //     Level::Info => "INFO",
                //     Level::Debug => "DEBUG",
                //     Level::Trace => "TRACE",
                // };

                let location = false;
                let module = false;

                let mut log = format!("{level} {}", record.args());

                if location {
                    log = format!(
                        "[{}::{}] {log}",
                        record.file().unwrap_or_default(),
                        record.line().unwrap_or_default()
                    );
                }

                if module {
                    log = format!("{} {log}", record.module_path().unwrap_or_default());
                }

                writeln!(f, "{log}")
            })
            .init();
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
    pub async fn start<T: View + Default + Sized + 'static>() -> Result<()> {
        Window::start(Self::new(Own::<T>::default())).await
    }

    #[cfg(target_os = "android")]
    pub async fn start(first_view: Own<dyn View>, app: crate::AndroidApp) -> Result<()> {
        dbg!("PENIJEE");

        use winit::platform::android::EventLoopBuilderExtAndroid;

        // android_logger::try_init(android_logger::Config::default().
        // with_max_level(LevelFilter::Trace));

        // try_init();

        // android_logger::try_init(android_logger::Config::default().
        // with_max_level(LevelFilter::Trace));

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
        Window::current().set_title(title);
    }

    pub async fn set_window_size(size: impl Into<Size<u32>> + Send + 'static) {
        from_main(|| {
            Window::current().set_size(size);
        })
        .await;
        sleep(Duration::from_secs_f32(0.05)).await;
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

impl window::App for App {
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
        let window_size = UIManager::resolution();

        if window_size.has_no_area() {
            return;
        }

        pass.set_viewport(0.0, 0.0, window_size.width, window_size.height, 0.0, 1.0);

        LevelDrawer::draw(pass);
        UI::draw(pass);
    }

    fn resize(&mut self, _position: Point, size: Size<u32>) {
        UIManager::root_view_weak().set_size(size); //.set_origin(position);
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
