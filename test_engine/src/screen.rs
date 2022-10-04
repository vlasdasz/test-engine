#![allow(clippy::mismatched_target_os)]

use std::{ops::DerefMut, path::Path, ptr::null_mut};

use gl_image::ImageShaders;
use gl_wrapper::{buffers::Buffers, monitor::Monitor, GLWrapper};
#[cfg(desktop)]
use gl_wrapper::{gl_events::GlEvents, GLFWManager};
use gm::{flat::Size, volume::GyroData, Color};
use net::API;
use rtools::{Dispatch, Time, Unwrap};
use sprites::{get_sprites_drawer, set_sprites_drawer, Player};
use ui::{
    layout::Placer,
    refs::{Own, ToWeak, Weak},
    UIManager, View, ViewCallbacks, ViewFrame, ViewLayout,
};

use crate::{
    app::TestEngineAction, assets::Assets, sprites_drawer::TESpritesDrawer, ui_drawer::TEUIDrawer,
    ui_layer::UILayer,
};

static mut SCREEN: *mut Screen = null_mut();

pub struct Screen {
    pub ui: Own<UILayer>,

    #[cfg(desktop)]
    glfw:    GLFWManager,
    monitor: Unwrap<Monitor>,
}

impl Screen {
    pub fn player(&self) -> Weak<Player> {
        if let Some(level) = &self.ui.level {
            level.player()
        } else {
            Default::default()
        }
    }

    pub fn add_monitor(&mut self, monitor: Monitor) {
        self.monitor = Unwrap::new(monitor);
        UIManager::set_screen_scale(self.monitor.scale);
    }

    #[cfg(desktop)]
    fn setup_events(&mut self) {
        self.ui.setup_events();

        let mut this = self.weak();
        GlEvents::get().size_changed.sub(move |size| {
            this.on_size_changed(size);
        });

        GlEvents::get().frame_drawn.sub(move |_| {
            this.update();
        });
    }

    fn init(&mut self, _size: Size, view: Own<dyn View>) {
        #[cfg(desktop)]
        {
            let m = self.glfw.monitors.first().unwrap().clone();
            self.add_monitor(m);
        }

        self.ui.debug_view.place = Placer::new(self.ui.debug_view.weak_view()).into();
        self.ui.debug_view.init_views();
        self.ui.debug_view.setup();

        GLWrapper::enable_blend();
        GLWrapper::set_clear_color(Color::GRAY);

        UIManager::set_view(view);

        #[cfg(desktop)]
        {
            let size = Screen::adjust_size(self.monitor.clone(), _size);
            self.set_size(size);
        }
    }

    #[cfg(not(any(mobile, macos)))]
    fn adjust_size(monitor: Monitor, size: Size) -> Size {
        size * monitor.scale
    }

    #[cfg(macos)]
    fn adjust_size(_monitor: Monitor, size: Size) -> Size {
        size
    }
}

impl Screen {
    pub fn current() -> &'static mut Screen {
        unsafe {
            if SCREEN.is_null() {
                panic!("Assets were not initialized");
            }
            SCREEN.as_mut().unwrap()
        }
    }

    fn calculate_fps(&mut self) {
        let now = Time::now();

        let interval = now - self.ui.prev_time;
        self.ui.prev_time = now;

        self.ui.frame_time = interval as f64 / 1000000000.0;
        self.ui.fps = (1.0 / self.ui.frame_time as f64) as u64;

        let fps = self.ui.fps;
        self.ui.debug_view.fps.set(fps);
        if API::is_ok() {
            self.ui.debug_view.url.set(API::base_url().to_string());
        } else {
            self.ui.debug_view.url.set("API not initizlized");
        }
    }

    pub fn update(&mut self) -> TestEngineAction {
        self.calculate_fps();

        UIManager::drawer().reset_viewport();
        UIManager::remove_scheduled();
        UIManager::set_scheduled();

        GLWrapper::clear();

        if self.ui.level.is_some() {
            self.update_level();
        }

        let view = UIManager::root_view();

        view.calculate_frames();
        UIManager::drawer().update(view);
        UIManager::drawer().draw(view);

        self.ui.debug_view.calculate_frames();
        UIManager::drawer().update(self.ui.debug_view.deref_mut());
        UIManager::drawer().draw(self.ui.debug_view.deref_mut());

        Dispatch::call();

        #[cfg(desktop)]
        self.glfw.swap_buffers();

        // TODO: tis ugly
        if UIManager::get().close_keyboard {
            UIManager::get().close_keyboard = false;
            TestEngineAction::CloseKeyboard
        } else if UIManager::get().open_keyboard {
            UIManager::get().open_keyboard = false;
            TestEngineAction::OpenKeyboard
        } else {
            TestEngineAction::None
        }
    }

    fn update_level(&mut self) {
        let cursor_position = self.ui.cursor_position.clone();

        let Some(level) = &mut self.ui.level else {
            return;
        };

        level.base_mut().update_physics();
        level.update();

        level.set_cursor_position(cursor_position);

        for sprite in level.sprites_mut() {
            sprite.update();
            sprite.draw();
        }
    }

    pub fn set_size(&mut self, size: Size) -> &mut Self {
        #[cfg(desktop)]
        self.glfw.set_size(size);
        self.on_size_changed(size);
        self
    }

    fn on_size_changed(&mut self, size: Size) {
        trace!("Size changed: {:?}", size);
        UIManager::root_view().set_frame(size);
        get_sprites_drawer().set_resolution(size);
        get_sprites_drawer().set_camera_position((0, 0).into());
        self.update();
    }

    pub(crate) fn on_gyro_changed(&mut self, gyro: GyroData) {
        error!("GyroData: {:?}", gyro);
        let Some(level) = &mut self.ui.level else {
            return;
        };
        level.on_gyro_changed(gyro);
    }

    #[cfg(desktop)]
    pub fn start_main_loop(&mut self) {
        self.setup_events();
        self.glfw.start_main_loop()
    }
}

impl Screen {
    pub fn new(size: impl Into<Size> + Clone, assets_path: &Path, view: Own<dyn View>) -> Own<Self> {
        trace!("Creating screen");

        #[cfg(desktop)]
        let glfw = GLFWManager::default();
        #[cfg(desktop)]
        trace!("GLFWManager: OK");

        Assets::init(assets_path);
        trace!("Assets: Ok");

        let ui = Own::<UILayer>::default();
        trace!("UILayer: OK");

        UIManager::set_drawer(Own::<TEUIDrawer>::default());
        trace!("UIDrawer: OK");

        set_sprites_drawer(TESpritesDrawer::new());
        trace!("SpritesDrawer: OK");

        let mut screen = Own::new(Self {
            ui,
            #[cfg(desktop)]
            glfw,
            monitor: Default::default(),
        });

        unsafe {
            SCREEN = screen.deref_mut() as *mut Screen;
        }

        Buffers::init(Buffers::default());
        ImageShaders::init(ImageShaders::default());

        screen.init(size.into(), view);

        screen
    }
}
