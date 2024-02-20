use std::{
    future::Future,
    io::Write,
    ops::{Deref, DerefMut},
    ptr::null_mut,
};

use anyhow::Result;
use dispatch::{from_main, invoke_dispatched};
use env_logger::Builder;
use gm::{
    flat::{Point, Rect, Size},
    Color,
};
use log::{trace, warn, Level, LevelFilter};
use manage::data_manager::DataManager;
use refs::{Own, Rglica, Weak};
use rtools::sleep;
use tokio::spawn;
use ui::{
    check_touch, Container, Touch, TouchEvent, TouchStack, UIEvents, UIManager, View, ViewAnimation,
    ViewData, ViewFrame, ViewLayout, ViewSetup, ViewSubviews, ViewTest, WeakView,
};
use ui_views::{DrawingView, ImageView, Label};
use vents::OnceEvent;
use wgpu::{PolygonMode, RenderPass};
use wgpu_text::glyph_brush::{BuiltInLineBreaker, HorizontalAlign, Layout, Section, Text, VerticalAlign};
use wgpu_wrapper::{ElementState, Font, MouseButton, Screenshot, State, WGPUApp, WGPUDrawer};
use winit::{
    event::{KeyEvent, TouchPhase},
    keyboard,
};

use crate::assets::Assets;

static mut APP: *mut App = null_mut();

pub struct App {
    root_view:    WeakView,
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

    pub fn state() -> &'static State {
        &Self::current().wgpu_app.state
    }

    pub fn root_view_size() -> Size {
        Self::current().root_view.size()
    }

    fn make_app(first_view: Own<dyn View>) -> Box<Self> {
        Builder::from_default_env()
            .filter_level(LevelFilter::Debug)
            .filter_module("wgpu_core::device", LevelFilter::Warn)
            .filter_module("wgpu_core::present", LevelFilter::Warn)
            .filter_module("wgpu_core::resource", LevelFilter::Warn)
            .filter_module("wgpu_core::instance", LevelFilter::Warn)
            .filter_module("wgpu_hal::metal::surface", LevelFilter::Warn)
            .filter_module("wgpu_hal::metal::device", LevelFilter::Warn)
            .filter_module("wgpu_hal::metal", LevelFilter::Warn)
            .filter_module("naga::front", LevelFilter::Warn)
            .filter_module("naga::proc::constant_evaluator", LevelFilter::Warn)
            .filter_module("naga::valid::interface", LevelFilter::Warn)
            .filter_module("naga::valid::function", LevelFilter::Warn)
            .format(|f, record| {
                let level = match record.level() {
                    Level::Error => "🔴",
                    Level::Warn => "🟡",
                    Level::Info => "🟢",
                    Level::Debug => "🔵",
                    Level::Trace => "⚪",
                };

                let location = false;

                let mut log = format!("{level} {}", record.args());

                if location {
                    log = format!(
                        "[{}::{}] {log}",
                        record.file().unwrap_or_default(),
                        record.line().unwrap_or_default()
                    );
                }

                writeln!(f, "{log}")
            })
            .init();

        #[cfg(desktop)]
        Assets::init(crate::git_root().expect("git_root()"));
        #[cfg(mobile)]
        Assets::init(std::path::PathBuf::default());
        let mut app = Self::new(first_view);
        unsafe {
            assert!(APP.is_null(), "Another App already exists");
            APP = std::ptr::from_mut(app.as_mut());
        }
        app
    }

    pub async fn start(first_view: Own<dyn View>) -> Result<()> {
        WGPUApp::start(Self::make_app(first_view)).await
    }

    pub async fn start_with_actor(
        first_view: Own<dyn View>,
        actions: impl Future<Output = Result<()>> + Send + 'static,
    ) -> Result<()> {
        let app = Self::make_app(first_view);

        spawn(async move {
            let recv = from_main(|| App::current().window_ready.once_async()).await;
            recv.await.unwrap();
            let _ = actions.await;
            WGPUApp::close();
        });

        WGPUApp::start(app).await
    }

    pub async fn set_test_view<T: View + ViewTest + Default + 'static>(width: u32, height: u32) -> Weak<T> {
        let view = from_main(move || {
            let view = T::new();
            let weak = view.weak();
            let mut root = UIManager::root_view();
            root.remove_all_subviews();
            let view = root.add_subview(view);
            view.place().back();
            trace!("{width} - {height}");
            App::set_window_size((width, height));
            weak
        })
        .await;

        sleep(0.1);

        view
    }

    fn new(first_view: Own<dyn View>) -> Box<Self> {
        Box::new(Self {
            cursor_position: Default::default(),
            root_view:       UIManager::root_view(),
            first_view:      first_view.into(),
            window_ready:    Default::default(),
            wgpu_app:        Default::default(),
        })
    }

    fn rescale_frame(rect: &Rect, display_scale: f32) -> Rect {
        rect * display_scale
    }

    fn update_view(view: &mut dyn View) {
        if view.is_hidden() {
            return;
        }
        view.layout();
        view.commit_animations();
        view.calculate_absolute_frame();
        view.update();
        view.trigger_events();
        for view in view.subviews_mut() {
            Self::update_view(view.deref_mut());
        }
    }

    fn draw_view<'a>(
        pass: &mut RenderPass<'a>,
        drawer: &'a WGPUDrawer,
        view: &'a dyn View,
        sections: &mut Vec<Section<'a>>,
    ) {
        const DRAW_DEBUG_FRAMES: bool = true;

        if view.is_hidden() {
            return;
        }

        if view.absolute_frame().size.is_invalid() {
            warn!(
                "View has invalid frame: {}. Frame: {:?} ",
                view.label(),
                view.frame()
            );
            return;
        }

        let frame = Self::rescale_frame(view.absolute_frame(), 1.0);

        if !frame.origin.positive() {
            return;
        }

        drawer.draw_rect(pass, &frame, view.color(), PolygonMode::Fill);

        if let Some(image_view) = view.as_any().downcast_ref::<ImageView>() {
            if image_view.image.is_ok() {
                let image = image_view.image;
                // let size: Size = image.size.into();
                // let frame = &size.fit_in_rect::<{ Axis::X }>(view.absolute_frame());
                // let frame = Self::rescale_frame(frame, 1.0, drawer.window_size);

                drawer.draw_image(pass, image.get_static(), &frame);
            }
        } else if let Some(label) = view.as_any().downcast_ref::<Label>()
            && !label.text.is_empty()
        {
            let center = frame.center();

            let section = Section::default()
                .add_text(
                    Text::new(&label.text)
                        .with_scale(label.text_size())
                        .with_color(label.text_color().as_slice()),
                )
                .with_bounds((frame.width(), frame.height()))
                .with_layout(
                    Layout::default()
                        .v_align(VerticalAlign::Center)
                        .h_align(HorizontalAlign::Center)
                        .line_breaker(BuiltInLineBreaker::UnicodeLineBreaker),
                )
                .with_screen_position((center.x, center.y));

            sections.push(section);
        } else if let Some(drawing_view) = view.as_any().downcast_ref::<DrawingView>() {
            for path in drawing_view.paths() {
                drawer.draw_buffer(
                    pass,
                    &frame,
                    path.mode,
                    path.buffer(),
                    path.bind_group(),
                    path.vertex_range(),
                );
            }
        }

        if DRAW_DEBUG_FRAMES {
            drawer.draw_rect(pass, &frame, &Color::TURQUOISE, PolygonMode::Line);
        }

        for view in view.subviews() {
            if view.dont_hide() || view.absolute_frame().intersects(UIManager::root_view().frame()) {
                Self::draw_view(pass, drawer, view.deref(), sections)
            }
        }
    }

    pub fn process_touch_event(&mut self, mut touch: Touch) -> bool {
        const LOG_TOUCHES: bool = false;

        if UIManager::touch_disabled() {
            return false;
        }

        UIEvents::on_touch().trigger(touch);

        if LOG_TOUCHES && !touch.is_moved() {
            warn!("{touch:?}");
        }

        if UIManager::display_touches() && !touch.is_moved() {
            let mut view = Container::new();
            view.set_size((5, 5)).set_color(Color::random());
            view.set_center(touch.position);
            UIManager::root_view().add_subview(view);
        }

        let _level_touch = touch;
        // TODO: Revisit scale
        // if Platform::DESKTOP {
        //     touch.position = self.cursor_position / UIManager::ui_scale();
        // } else {
        //     touch.position /= UIManager::ui_scale();
        // }

        for view in TouchStack::touch_views() {
            if check_touch(view, &mut touch) {
                return true;
            }
        }

        // if let Some(level) = &mut self.level {
        //     level.set_cursor_position(level_touch.position);
        //     if touch.is_began() {
        //         level.add_touch(level_touch.position)
        //     }
        // }

        false
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

    pub fn on_char(&mut self, ch: char) {
        UIManager::keymap().check(ch);
    }
}

impl wgpu_wrapper::App for App {
    fn window_ready(&mut self) {
        let view = UIManager::root_view().add_subview(self.first_view.take().unwrap());
        view.place().back();
        self.update();
        self.window_ready.trigger(());
    }

    fn update(&mut self) {
        UIManager::free_deleted_views();
        invoke_dispatched();
        Self::update_view(UIManager::root_view().deref_mut())
    }

    fn render<'a>(&'a mut self, pass: &mut RenderPass<'a>, drawer: &'a WGPUDrawer) {
        let mut sections: Vec<Section> = vec![];
        Self::draw_view(pass, drawer, self.root_view.deref(), &mut sections);

        Font::helvetice().brush.queue(&drawer.device, &drawer.queue, sections).unwrap()
    }

    fn resize(&mut self, _position: Point, size: Size<u32>) {
        UIManager::root_view().set_size(size); //.set_origin(position);
        UIEvents::size_changed().trigger(size);
        self.update();
    }

    fn mouse_moved(&mut self, position: Point) -> bool {
        self.cursor_position = position;
        self.process_touch_event(Touch {
            id: 1,
            position,
            event: TouchEvent::Moved,
            button: MouseButton::Left,
        })
    }

    fn mouse_event(&mut self, state: ElementState, button: MouseButton) -> bool {
        self.process_touch_event(Touch {
            id: 1,
            position: self.cursor_position,
            event: state.into(),
            button,
        })
    }

    fn touch_event(&mut self, touch: winit::event::Touch) -> bool {
        self.process_touch_event(Touch {
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

        #[allow(clippy::single_match)]
        match event.logical_key {
            keyboard::Key::Character(st) => {
                self.on_char(st.to_string().chars().last().unwrap());
            }
            _ => (),
        }
    }

    fn set_wgpu_app(&mut self, app: Rglica<WGPUApp>) {
        self.wgpu_app = app;
    }
}
