use std::{
    ops::{Deref, DerefMut},
    rc::Rc,
};

use cfg_if::cfg_if;
use chrono::Utc;
#[cfg(not(any(target_os = "ios", target_os = "android")))]
use gl_wrapper::{events::Events, GLDrawer};
use gl_wrapper::{monitor::Monitor, GLWrapper};
use gm::{Color, Size};
use rtools::{ToRglica, Unwrap};
use sprites::{Sprite, SpritesDrawer};

use crate::{assets::Assets, paths, sprites_drawer::TESpritesDrawer, ui_layer::UILayer};

pub struct Screen {
    pub ui: UILayer,

    #[cfg(not(any(target_os = "ios", target_os = "android")))]
    pub events: Box<Events>,

    #[cfg(not(any(target_os = "ios", target_os = "android")))]
    drawer:         GLDrawer,
    monitor:        Unwrap<Monitor>,
    sprites_drawer: Box<dyn SpritesDrawer>,
}

impl Screen {
    pub fn add_monitor(&mut self, monitor: Monitor) {
        self.monitor = monitor.into();
        self.ui.drawer.set_scale(self.monitor.scale);
    }

    #[cfg(not(any(target_os = "ios", target_os = "android")))]
    fn setup_events(&mut self) {
        self.ui.setup_events();

        let mut this = self.to_rglica();
        self.events.on_size_changed.subscribe(move |size| {
            this.on_size_changed(size);
        });

        let mut this = self.to_rglica();
        self.events.on_frame_drawn.subscribe(move |_| this.update());
    }

    fn init(&mut self, size: Size) {
        dbg!(&size);

        cfg_if! { if #[cfg(not(any(target_os="ios", target_os="android")))] {
            let monitor = self.drawer.monitors.first().expect("BUG: failed to get monitor").clone();
            self.add_monitor(monitor);
        }}

        GLWrapper::enable_blend();
        GLWrapper::set_clear_color(&Color::GRAY);

        self.ui.root_view.calculate_frames();

        cfg_if! { if #[cfg(not(any(target_os="ios", target_os="android")))] {
            let size = Screen::adjust_size(self.monitor.clone(), size);
            self.set_size(size);
        }}
    }

    #[cfg(not(target_os = "macos"))]
    fn adjust_size(monitor: Monitor, size: Size) -> Size {
        dbg!(size * monitor.scale)
    }

    #[cfg(target_os = "macos")]
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

        let drawer = self.sprites_drawer.deref();

        drawer.set_camera_position(level.player().position());

        for sprite in level.sprites_mut() {
            sprite.update();
            sprite.draw();
        }

        level.player().update();
        level.player().draw();
    }

    pub fn set_size(&mut self, size: Size) -> &mut Self {
        dbg!(&size);
        #[cfg(not(any(target_os = "ios", target_os = "android")))]
        self.drawer.set_size(size);
        self.on_size_changed(size);
        error!("Debug: {:?}", self.ui.root_view.frame());
        self
    }

    fn on_size_changed(&mut self, size: Size) {
        self.ui.drawer.set_size(size);
        self.ui.root_view.set_frame(size.into());
        self.sprites_drawer.set_resolution(size);
        self.sprites_drawer.set_camera_position((0, 0).into());
        self.update();
    }

    #[cfg(not(any(target_os = "ios", target_os = "android")))]
    pub fn start_main_loop(&mut self) {
        self.setup_events();
        self.drawer.start_main_loop()
    }
}

impl Screen {
    pub fn new(size: Size) -> Self {
        ui::set_default_font_path(paths::fonts().join("SF.otf"));

        #[cfg(not(any(target_os = "ios", target_os = "android")))]
        let events = Box::new(Events::default());

        #[cfg(not(any(target_os = "ios", target_os = "android")))]
        let drawer = GLDrawer::new(events.to_rglica());
        let assets = Rc::new(Assets::default());
        let sprites_drawer: Box<dyn SpritesDrawer> = TESpritesDrawer::new(assets.clone());

        let mut ui = UILayer::new(assets, sprites_drawer.to_rglica());

        cfg_if! {if #[cfg(not(any(target_os = "ios", target_os = "android")))] {
            ui.events = events.to_rglica();
        }}

        let mut screen = Self {
            ui,
            #[cfg(not(any(target_os = "ios", target_os = "android")))]
            events,
            #[cfg(not(any(target_os = "ios", target_os = "android")))]
            drawer,
            sprites_drawer,
            monitor: Default::default(),
        };

        screen.init(size);

        screen
    }
}
