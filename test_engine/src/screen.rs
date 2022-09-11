#![allow(clippy::mismatched_target_os)]

use std::{ops::DerefMut, path::Path};

#[cfg(desktop)]
use gl_wrapper::{gl_events::GlEvents, GLFWManager};
use gl_wrapper::{monitor::Monitor, GLWrapper};
use gm::{flat::Size, volume::GyroData, Color};
use rtools::{Dispatch, Time, Unwrap};
use ui::{UIDrawer, ViewData, ViewFrame, ViewLayout};

use crate::{assets::Assets, ui_layer::UILayer};

pub struct Screen {
    pub ui: Box<UILayer>,

    #[cfg(desktop)]
    glfw:    GLFWManager,
    monitor: Unwrap<Monitor>,
}

impl Screen {
    pub fn add_monitor(&mut self, monitor: Monitor) {
        self.monitor = monitor.into();
        self.ui.drawer.set_screen_scale(self.monitor.scale);
    }

    #[cfg(desktop)]
    fn setup_events(&mut self) {
        self.ui.setup_events();

        GlEvents::get().size_changed.set(self, |this, size| {
            this.on_size_changed(size);
        });

        GlEvents::get().frame_drawn.set(self, |this, _| this.update());
    }

    fn init(mut self, _size: Size) -> Self {
        #[cfg(desktop)]
        {
            let m = self.glfw.monitors.first().unwrap().clone();
            self.add_monitor(m);
        }

        GLWrapper::enable_blend();
        GLWrapper::set_clear_color(Color::GRAY);

        self.ui.root_view.calculate_frames();
        self.ui.root_view.set_drawer(self.ui.drawer.rglica());

        #[cfg(desktop)]
        {
            let size = Screen::adjust_size(self.monitor.clone(), _size);
            self.set_size(size);
        }

        self
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
    fn calculate_fps(&mut self) {
        let now = Time::now();

        let interval = now - self.ui.prev_time;
        self.ui.prev_time = now;

        self.ui.frame_time = interval as f64 / 1000000000.0;
        self.ui.fps = (1.0 / self.ui.frame_time as f64) as u64;

        if self.ui.debug_view.is_ok() {
            self.ui.debug_view.fps.set(self.ui.fps);
        }
    }

    pub fn update(&mut self) {
        self.calculate_fps();

        GLWrapper::clear();

        if self.ui.view.is_ok() && self.ui.view.level().is_ok() {
            self.update_level();
        }

        self.ui.drawer.update(self.ui.root_view.deref_mut());
        if self.ui.view.is_ok() {
            self.ui
                .view
                .set_frame(self.ui.root_view.frame().with_zero_origin());
        }
        self.ui.root_view.calculate_frames();
        self.ui.drawer.draw(self.ui.root_view.deref_mut());

        self.ui.drawer.reset_viewport();

        Dispatch::call();
        self.ui.root_view.remove_scheduled();
    }

    fn update_level(&mut self) {
        if self.ui.view.is_null() {
            return;
        }

        let mut level = self.ui.view.level();

        level.base_mut().update_physics();
        level.update();

        level.set_cursor_position(self.ui.cursor_position);

        for sprite in level.sprites_mut() {
            sprite.update();
            sprite.draw();
        }
    }

    pub fn set_size(&mut self, size: Size) -> &mut Self {
        #[cfg(desktop)]
        self.glfw.set_size(size);
        self.on_size_changed(size);
        trace!("Screen size: {:?}", self.ui.root_view.frame());
        self
    }

    fn on_size_changed(&mut self, size: Size) {
        self.ui.drawer.set_size(size);
        self.ui.root_view.set_frame(size);
        self.ui.sprites_drawer.set_resolution(size);
        self.ui.sprites_drawer.set_camera_position((0, 0).into());
        self.update();
    }

    pub(crate) fn on_gyro_changed(&mut self, gyro: GyroData) {
        error!("GyroData: {:?}", gyro);
        if self.ui.view.level().is_ok() {
            self.ui.view.level().on_gyro_changed(gyro)
        }
    }

    #[cfg(desktop)]
    pub fn start_main_loop(&mut self) {
        self.setup_events();
        self.glfw.start_main_loop()
    }
}

impl Screen {
    pub fn new(size: impl Into<Size>, assets_path: &Path) -> Self {
        trace!("Creating screen");

        #[cfg(desktop)]
        let glfw = GLFWManager::new();
        #[cfg(desktop)]
        trace!("GLFWManager: OK");

        Assets::init(assets_path);
        trace!("Assets: Ok");

        let ui = UILayer::new();
        trace!("UILayer: OK");

        Self {
            ui,
            #[cfg(desktop)]
            glfw,
            monitor: Default::default(),
        }
        .init(size.into())
    }
}
