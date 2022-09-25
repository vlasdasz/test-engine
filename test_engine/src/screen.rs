#![allow(clippy::mismatched_target_os)]

use std::{ops::DerefMut, path::Path, ptr::null_mut};

#[cfg(desktop)]
use gl_wrapper::{gl_events::GlEvents, GLFWManager};
use gl_wrapper::{monitor::Monitor, GLWrapper};
use gm::{flat::Size, volume::GyroData, Color};
use net::API;
use rtools::{Dispatch, Rglica, Time, ToRglica, UnwrapBox};
use sprites::{get_sprites_drawer, set_sprites_drawer, Player};
use ui::{get_ui_drawer, layout::Placer, set_ui_drawer, View, ViewCallbacks, ViewFrame, ViewLayout};

use crate::{
    app::TestEngineAction, assets::Assets, sprites_drawer::TESpritesDrawer, ui_drawer::TEUIDrawer,
    ui_layer::UILayer,
};

static mut SCREEN: *mut Screen = null_mut();

pub struct Screen {
    pub ui: Box<UILayer>,

    #[cfg(desktop)]
    glfw:    GLFWManager,
    monitor: UnwrapBox<Monitor>,
}

impl Screen {
    pub fn player(&self) -> Rglica<Player> {
        if let Some(level) = &self.ui.level {
            level.player()
        } else {
            Default::default()
        }
    }

    pub fn add_monitor(&mut self, monitor: Monitor) {
        self.monitor = monitor.into();
        get_ui_drawer().set_screen_scale(self.monitor.scale);
    }

    #[cfg(desktop)]
    fn setup_events(&mut self) {
        self.ui.setup_events();

        GlEvents::get().size_changed.set(self, |this, size| {
            this.on_size_changed(size);
        });

        GlEvents::get().frame_drawn.set(self, |this, _| {
            this.update();
        });
    }

    fn init(&mut self, _size: Size, view: Box<dyn View>) {
        #[cfg(desktop)]
        {
            let m = self.glfw.monitors.first().unwrap().clone();
            self.add_monitor(m);
        }

        self.ui.debug_view.place = Placer::new(self.ui.debug_view.rglica()).into();
        self.ui.debug_view.init_views();
        self.ui.debug_view.setup();

        GLWrapper::enable_blend();
        GLWrapper::set_clear_color(Color::GRAY);

        get_ui_drawer().set_view(view);

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

        self.ui.debug_view.fps.set(self.ui.fps);
        self.ui.debug_view.url.set(API::base_url().to_string());
    }

    pub fn update(&mut self) -> TestEngineAction {
        self.calculate_fps();

        get_ui_drawer().reset_viewport();
        get_ui_drawer().remove_scheduled();
        get_ui_drawer().set_scheduled();

        GLWrapper::clear();

        if self.ui.level.is_some() {
            self.update_level();
        }

        let view = get_ui_drawer().root_view();

        view.calculate_frames();
        get_ui_drawer().update(view);
        get_ui_drawer().draw(view);

        self.ui.debug_view.calculate_frames();
        get_ui_drawer().update(self.ui.debug_view.deref_mut());
        get_ui_drawer().draw(self.ui.debug_view.deref_mut());

        Dispatch::call();

        #[cfg(desktop)]
        self.glfw.swap_buffers();

        // TODO: tis ugly
        if *get_ui_drawer().close_keyboard() {
            *get_ui_drawer().close_keyboard() = false;
            TestEngineAction::CloseKeyboard
        } else if *get_ui_drawer().open_keyboard() {
            *get_ui_drawer().open_keyboard() = false;
            TestEngineAction::OpenKeyboard
        } else {
            TestEngineAction::None
        }
    }

    fn update_level(&mut self) {
        let Some(level) = &mut self.ui.level else {
            return;
        };

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
        self
    }

    fn on_size_changed(&mut self, size: Size) {
        trace!("Size changed: {:?}", size);
        get_ui_drawer().root_view().set_frame(size);
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
    pub fn new(size: impl Into<Size> + Clone, assets_path: &Path, view: Box<dyn View>) -> Box<Self> {
        trace!("Creating screen");

        #[cfg(desktop)]
        let glfw = GLFWManager::default();
        #[cfg(desktop)]
        trace!("GLFWManager: OK");

        Assets::init(assets_path);
        trace!("Assets: Ok");

        let ui = Box::<UILayer>::default();
        trace!("UILayer: OK");

        set_ui_drawer(Box::<TEUIDrawer>::default());
        trace!("UIDrawer: OK");

        set_sprites_drawer(TESpritesDrawer::new());
        trace!("SpritesDrawer: OK");

        let mut screen = Box::new(Self {
            ui,
            #[cfg(desktop)]
            glfw,
            monitor: Default::default(),
        });

        unsafe {
            SCREEN = screen.to_rglica().as_ptr();
        }

        screen.init(size.into(), view);

        screen
    }
}
