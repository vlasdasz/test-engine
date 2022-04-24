#![allow(clippy::mismatched_target_os)]

use std::{ops::DerefMut, path::Path, rc::Rc};

use cfg_if::cfg_if;
use chrono::Utc;
#[cfg(desktop)]
use gl_wrapper::{events::Events, GLFWManager};
use gl_wrapper::{monitor::Monitor, GLWrapper};
use gm::{flat::Size, volume::GyroData, Color};
use rtools::{ToRglica, Unwrap};
use sprites::SpritesDrawer;
use ui::ViewFrame;

use crate::{assets::Assets, sprites_drawer::TESpritesDrawer, ui_layer::UILayer};

pub struct Screen {
    pub ui: Box<UILayer>,

    #[cfg(desktop)]
    pub events: Box<Events>,

    #[cfg(desktop)]
    glfw:           GLFWManager,
    monitor:        Unwrap<Monitor>,
    sprites_drawer: Box<dyn SpritesDrawer>,
}

impl Screen {
    pub fn add_monitor(&mut self, monitor: Monitor) {
        self.monitor = monitor.into();
        self.ui.drawer.set_screen_scale(self.monitor.scale);
    }

    #[cfg(desktop)]
    fn setup_events(&mut self) {
        self.ui.setup_events();

        self.events.on_size_changed.set(self, |size, this| {
            this.on_size_changed(size);
        });

        self.events.on_frame_drawn.set(self, |_, this| this.update());
    }

    fn init(&mut self, _size: Size) {
        #[cfg(desktop)]
        {
            let m = self.glfw.monitors.first().unwrap().clone();
            self.add_monitor(m);
        }

        GLWrapper::enable_blend();
        GLWrapper::set_clear_color(&Color::GRAY);

        self.ui.root_view.calculate_frames();

        #[cfg(desktop)]
        {
            let size = Screen::adjust_size(self.monitor.clone(), _size);
            self.set_size(size);
        }
    }

    #[cfg(not(any(mobile, macos)))]
    fn adjust_size(monitor: Monitor, size: Size) -> Size {
        dbg!(size * monitor.scale)
    }

    #[cfg(macos)]
    fn adjust_size(_monitor: Monitor, size: Size) -> Size {
        size
    }
}

impl Screen {
    fn calculate_fps(&mut self) {
        let now = Utc::now().timestamp_nanos();

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

        self.update_level();

        self.ui.drawer.update(self.ui.root_view.deref_mut());
        self.ui.root_view.calculate_frames();
        self.ui.drawer.draw(self.ui.root_view.deref_mut());

        self.ui.drawer.reset_viewport();
    }

    fn update_level(&mut self) {
        if self.ui.view.is_null() {
            return;
        }

        let level = self.ui.view.level_mut();

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
        error!("Debug: {:?}", self.ui.root_view.frame());
        self
    }

    fn on_size_changed(&mut self, size: Size) {
        self.ui.drawer.set_size(size);
        self.ui.root_view.set_frame(size);
        self.sprites_drawer.set_resolution(size);
        self.sprites_drawer.set_camera_position((0, 0).into());
        self.update();
    }

    pub(crate) fn on_gyro_changed(&mut self, gyro: GyroData) {
        error!("GyroData: {:?}", gyro);
        self.ui.view.level_mut().on_gyro_changed(gyro)
    }

    #[cfg(desktop)]
    pub fn start_main_loop(&mut self) {
        self.setup_events();
        self.glfw.start_main_loop()
    }
}

impl Screen {
    pub fn new(assets_path: &Path, size: Size) -> Self {
        let mut assets = Assets::new(assets_path);

        ui::set_default_font_path(assets.paths.fonts.join("SF.otf"));

        #[cfg(desktop)]
        let events = Box::new(Events::default());

        #[cfg(desktop)]
        let glfw = GLFWManager::new(events.to_rglica());

        assets.init_gl_data();

        let assets = Rc::new(assets);

        let sprites_drawer: Box<dyn SpritesDrawer> = TESpritesDrawer::new(assets.clone());

        error!("Sprites Drawer: OK");

        error!("UILayer: OK");

        cfg_if! { if #[cfg(desktop)] {
            let mut ui = UILayer::new(assets, sprites_drawer.to_rglica());
            ui.events = events.to_rglica();
        } else {
            let ui = UILayer::new(assets, sprites_drawer.to_rglica());
        }};

        let mut screen = Self {
            ui,
            #[cfg(desktop)]
            events,
            #[cfg(desktop)]
            glfw,
            sprites_drawer,
            monitor: Default::default(),
        };

        screen.init(size);

        screen
    }
}
