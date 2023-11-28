use std::{ops::DerefMut, path::PathBuf, ptr::null_mut, sync::atomic::Ordering};

use chrono::Utc;
use dispatch::{from_main, on_main};
use gl_wrapper::{monitor::Monitor, GLWrapper};
#[cfg(desktop)]
use gl_wrapper::{system_events::SystemEvents, GLFWManager};
use gm::{
    flat::{IntSize, Rect},
    Color,
};
use rest::API;
use sprites::{get_sprites_drawer, set_sprites_drawer, Player};
use text::Font;
use ui::{
    refs::{assert_main_thread, Own, Weak},
    ToLabel, UIManager, View, ViewFrame, ViewSetup, ViewSubviews, ViewTest, MICROSECONDS_IN_ONE_SECOND,
};
use ui_views::debug_view::{DebugView, SHOW_DEBUG_VIEW};

use crate::{
    app::TestEngineAction, assets::Assets, sprites_drawer::TESpritesDrawer, ui_drawer::TEUIDrawer,
    ui_layer::UILayer,
};

static mut SCREEN: *mut Screen = null_mut();

pub struct Screen {
    pub(crate) ui: Own<UILayer>,

    #[cfg(desktop)]
    glfw:    GLFWManager,
    monitor: Monitor,
}

impl Screen {
    pub fn player(&self) -> Weak<Player> {
        if let Some(level) = &self.ui.level {
            level.player()
        } else {
            Default::default()
        }
    }

    #[cfg(desktop)]
    fn setup_events(&mut self) {
        self.ui.setup_events();

        let mut this = ui::refs::weak_from_ref(self);
        SystemEvents::get().size_changed.val(move |size| {
            this.size_changed(size);
        });

        SystemEvents::get().frame_drawn.sub(move || {
            this.update();
        });
    }

    fn init(&mut self, #[cfg(desktop)] window_size: IntSize, view: Own<dyn View>) {
        UIManager::set_display_scale(self.monitor.scale);

        GLWrapper::enable_blend();
        GLWrapper::enable_depth();
        GLWrapper::set_clear_color(Color::GRAY);

        UIManager::root_view().add_subview(view).place.as_background();

        if SHOW_DEBUG_VIEW.load(Ordering::Relaxed) {
            let mut debug_view = DebugView::new();
            debug_view.priority = 10;
            let weak = debug_view.weak();
            UIManager::root_view().add_subview(debug_view);
            Screen::current().ui.debug_view = weak;
        }
        #[cfg(desktop)]
        {
            self.glfw.set_size(window_size);
            self.size_changed(window_size);
        }
    }
}

impl Screen {
    pub fn current() -> &'static mut Screen {
        assert_main_thread();
        unsafe {
            assert!(!SCREEN.is_null(), "Screen was not initialized");
            SCREEN.as_mut().unwrap()
        }
    }

    #[cfg(desktop)]
    pub fn take_screenshot(path: impl ToString) {
        Self::current().glfw.take_screenshot(path)
    }

    #[cfg(mobile)]
    pub fn take_screenshot(_path: impl ToString) {
        todo!("Take screenshot is not implemented for mobile")
    }

    #[cfg(desktop)]
    pub fn set_title(title: impl ToLabel + Send + Sync + 'static) {
        on_main(move || Screen::current().glfw.set_window_title(&title.to_label()));
    }

    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    fn calculate_fps(&mut self) {
        let now = Utc::now().timestamp_micros();

        let interval = now - self.ui.prev_time;
        self.ui.prev_time = now;

        self.ui.frame_time = interval as f32 / MICROSECONDS_IN_ONE_SECOND as f32;
        self.ui.fps = (1.0 / self.ui.frame_time) as u64;

        if SHOW_DEBUG_VIEW.load(Ordering::Relaxed) && self.ui.debug_view.is_ok() {
            let fps = self.ui.fps;
            self.ui.debug_view.fps.trigger(fps);
            if API::is_ok() {
                self.ui.debug_view.set_custom("URL:", API::base_url());
            } else {
                self.ui.debug_view.set_custom("URL:", "API not initizlized");
            }
        }
    }

    pub fn update(&mut self) -> TestEngineAction {
        self.calculate_fps();

        UIManager::reset_viewport();

        GLWrapper::clear();

        if self.ui.level.is_some() {
            self.update_level();
        }

        let mut root_view = UIManager::root_view();
        let root_frame: Rect = UIManager::root_view_size().into();
        root_view.set_frame(root_frame);

        let mut drawer = UIManager::drawer();
        drawer.set_root_frame(root_frame);
        drawer.update(&mut root_view);
        drop(drawer);

        dispatch::invoke_dispatched();

        #[cfg(desktop)]
        self.glfw.swap_buffers();

        UIManager::update();

        // TODO: tis ugly
        if UIManager::get().close_keyboard.load(Ordering::Relaxed) {
            UIManager::get().close_keyboard.store(false, Ordering::Relaxed);
            TestEngineAction::CloseKeyboard
        } else if UIManager::get().open_keyboard.load(Ordering::Relaxed) {
            UIManager::get().open_keyboard.store(false, Ordering::Relaxed);
            TestEngineAction::OpenKeyboard
        } else {
            TestEngineAction::None
        }
    }

    fn update_level(&mut self) {
        let cursor_position = self.ui.cursor_position;

        let frame_time = self.ui.frame_time;

        let Some(level) = &mut self.ui.level else {
            return;
        };

        level.base_mut().update_physics(frame_time);
        level.update();

        level.set_cursor_position(cursor_position);

        for sprite in level.sprites_mut() {
            sprite.update();
            sprite.draw();
        }
    }

    #[cfg(desktop)]
    pub fn set_size(&mut self, size: impl Into<IntSize>) {
        self.glfw.set_size(size.into())
    }

    pub fn size_changed(&mut self, size: IntSize) {
        trace!("Size changed: {:?}", size);
        UIManager::set_window_size(size);
        get_sprites_drawer().set_resolution(size);
        get_sprites_drawer().set_camera_position((0, 0).into());
        Self::set_title(format!("{} x {}", size.width, size.height));
        self.update();
    }

    #[cfg(mobile)]
    pub(crate) fn on_gyro_changed(&mut self, gyro: gm::volume::GyroData) {
        // error!("GyroData: {:?}", gyro);

        ui::input::UIEvents::get().gyro_changed.trigger(gyro);

        let Some(level) = &mut self.ui.level else {
            return;
        };
        level.on_gyro_changed(gyro);
    }

    #[cfg(desktop)]
    pub fn start_main_loop(&mut self, callback: impl FnOnce()) -> anyhow::Result<()> {
        self.setup_events();
        self.glfw.start_main_loop(callback)
    }

    pub async fn set_test_view<T: View + ViewTest + Default + 'static>(width: u32, height: u32) {
        from_main(move || {
            let view = T::new();
            let mut root = UIManager::root_view();
            root.remove_all_subviews();
            let view = root.add_subview(view);
            view.place.back();
            #[cfg(desktop)]
            Screen::current().set_size((width, height));
        })
        .await
    }
}

impl Screen {
    pub fn new(
        monitor: Monitor,
        assets_path: impl Into<PathBuf>,
        root_view: Own<dyn View>,
        #[cfg(desktop)] glfw: GLFWManager,
        #[cfg(desktop)] window_size: IntSize,
    ) -> Own<Self> {
        trace!("Creating screen");

        Assets::init(assets_path);
        trace!("Assets: Ok");

        let ui = Own::<UILayer>::default();
        trace!("UILayer: OK");

        UIManager::set_drawer(TEUIDrawer::default());
        trace!("UIDrawer: OK");

        set_sprites_drawer(TESpritesDrawer::new());
        trace!("SpritesDrawer: OK");

        let mut screen = Own::new(Self {
            ui,
            #[cfg(desktop)]
            glfw,
            monitor,
        });

        unsafe {
            SCREEN = screen.deref_mut() as *mut Screen;
        }

        screen.init(
            #[cfg(desktop)]
            window_size,
            root_view,
        );

        screen
    }
}

impl Drop for Screen {
    fn drop(&mut self) {
        Font::san_francisco().free();
    }
}
