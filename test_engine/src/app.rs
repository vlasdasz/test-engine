use std::{any::type_name, future::Future, io::Write, path::PathBuf, ptr::null_mut};

use anyhow::Result;
use dispatch::{from_main, invoke_dispatched};
use env_logger::Builder;
use gm::flat::{Point, Size};
use level::LevelBase;
use log::{Level, LevelFilter};
use refs::{Own, Rglica};
use tokio::spawn;
use ui::{Touch, TouchEvent, UIEvents, UIManager, View, ViewData, ViewFrame, ViewSubviews};
use vents::OnceEvent;
use wgpu::RenderPass;
use wgpu_text::glyph_brush::Section;
use wgpu_wrapper::{ElementState, Font, MouseButton, Screenshot, WGPUApp, WGPUDrawer};
use winit::event::{KeyEvent, TouchPhase};

use crate::{
    assets::Assets,
    ui::{Input, UI},
};

static mut APP: *mut App = null_mut();

pub struct App {
    window_ready: OnceEvent,
    wgpu_app:     Rglica<WGPUApp>,

    pub(crate) first_view: Option<Own<dyn View>>,
    pub cursor_position:   Point,
}

impl App {
    pub fn current() -> &'static Self {
        unsafe {
            assert!(!APP.is_null(), "App was not initialized");
            APP.as_mut().unwrap()
        }
    }

    pub fn current_mut() -> &'static mut Self {
        unsafe {
            assert!(!APP.is_null(), "App was not initialized");
            APP.as_mut().unwrap()
        }
    }

    pub fn stop() {
        WGPUApp::close();
    }

    fn setup_log() {
        Builder::from_default_env()
            .filter_level(LevelFilter::Debug)
            .filter_module("winit::platform_impl::platform::app_state", LevelFilter::Error)
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

    fn new(first_view: Own<dyn View>) -> Box<Self> {
        Self::setup_log();

        #[cfg(desktop)]
        Assets::init(crate::git_root().expect("git_root()"));
        #[cfg(mobile)]
        Assets::init(std::path::PathBuf::default());
        let mut app = Box::new(Self {
            cursor_position: Default::default(),
            first_view:      first_view.into(),
            window_ready:    Default::default(),
            wgpu_app:        Default::default(),
        });
        unsafe {
            assert!(APP.is_null(), "Another App already exists");
            APP = std::ptr::from_mut(app.as_mut());
        }

        app
    }

    pub async fn start(first_view: Own<dyn View>) -> Result<()> {
        WGPUApp::start(Self::new(first_view)).await
    }

    pub async fn start_with_actor(
        first_view: Own<dyn View>,
        actions: impl Future<Output = Result<()>> + Send + 'static,
    ) -> Result<()> {
        let app = Self::new(first_view);

        spawn(async move {
            let recv = from_main(|| App::current().window_ready.val_async()).await;
            recv.await.unwrap();
            let _ = actions.await;
        });

        WGPUApp::start(app).await
    }

    pub fn set_window_title(title: impl ToString) {
        Self::current().wgpu_app.set_title(title);
    }

    pub fn set_window_size(size: impl Into<Size<u32>>) {
        Self::current().wgpu_app.set_window_size(size);
    }

    pub async fn take_screenshot() -> Result<Screenshot> {
        let recv = from_main(|| Self::current().wgpu_app.request_read_display()).await;
        let screenshot = recv.await?;
        Ok(screenshot)
    }

    pub fn fps() -> f32 {
        Self::current().wgpu_app.fps()
    }
}

impl wgpu_wrapper::App for App {
    fn window_ready(&mut self) {
        let view = UIManager::root_view_mut().__add_subview_internal(self.first_view.take().unwrap(), true);
        view.place().back();
        self.update();
        self.window_ready.trigger(());
    }

    fn update(&mut self) {
        UIManager::free_deleted_views();
        invoke_dispatched();
        UI::update();
    }

    fn render<'a>(&'a mut self, pass: &mut RenderPass<'a>, drawer: &'a WGPUDrawer) {
        let mut sections: Vec<Section> = vec![];
        UI::draw_view(pass, drawer, UIManager::root_view(), &mut sections, &mut 0.0);

        Font::helvetice()
            .brush
            .queue(WGPUApp::device(), WGPUApp::queue(), sections)
            .unwrap()
    }

    fn resize(&mut self, _position: Point, size: Size<u32>) {
        UIManager::root_view_mut().set_size(size); //.set_origin(position);
        UIEvents::size_changed().trigger(size);
        self.update();
    }

    fn mouse_moved(&mut self, position: Point) -> bool {
        self.cursor_position = position;
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

        if let Some(ch) = event.logical_key.to_text() {
            Input::on_char(ch.chars().last().unwrap());
        }
    }

    fn set_wgpu_app(&mut self, app: Rglica<WGPUApp>) {
        self.wgpu_app = app;
    }

    fn dropped_file(&mut self, path: PathBuf) {
        dbg!(type_name::<LevelBase>());
        UIManager::trigger_drop_file(path);
    }
}
