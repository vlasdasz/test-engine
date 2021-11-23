use std::{
    default::default,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use chrono::Utc;
#[cfg(not(any(target_os = "ios", target_os = "android")))]
use gl_wrapper::GLDrawer;
use gl_wrapper::GLWrapper;
#[cfg(not(any(target_os = "ios", target_os = "android")))]
use glfw::{Action, Key};
use gm::{Color, Point, Size};
use sprites::{Level, Sprite, SpritesDrawer};
use tools::{Boxed, Rglica, ToRglica};
#[cfg(not(any(target_os = "ios", target_os = "android")))]
use ui::input::touch::{ButtonState, Event};
use ui::{init_view_on, input::Touch, View, ViewBase};

use crate::{
    assets::Assets, debug_view::DebugView, paths, sprites_drawer::TESpritesDrawer,
    ui_drawer::UIDrawer,
};

pub trait GameView: View {
    fn level(&self) -> &dyn Level;
    fn level_mut(&mut self) -> &mut dyn Level;
    fn set_drawer(&mut self, _drawer: Rc<dyn SpritesDrawer>) {}
}

pub struct Screen {
    #[cfg(not(any(target_os = "ios", target_os = "android")))]
    cursor_position: Point,
    root_view:       Box<dyn View>,
    debug_view:      Rglica<DebugView>,
    view:            Rglica<dyn GameView>,
    #[cfg(not(any(target_os = "ios", target_os = "android")))]
    drawer:          GLDrawer,
    ui_drawer:       UIDrawer,
    sprites_drawer:  Rc<dyn SpritesDrawer>,
    fps:             u64,
    prev_time:       i64,
    frame_time:      f64,
}

impl Screen {
    pub fn on_touch(&mut self, mut touch: Touch) {
        self.root_view.check_touch(&mut touch);
    }

    pub fn add_view(mut self, mut view: Box<dyn GameView>) -> Self {
        let drawer = self.sprites_drawer.clone();
        view.set_drawer(drawer.clone());
        self.view = view.to_rglica();
        self.root_view.add_subview(view);
        self.view.level_mut().setup();
        self
    }

    pub fn add_debug_view(mut self) -> Self {
        self.debug_view = init_view_on::<DebugView>(self.root_view.deref_mut());
        //dbg!(&self.debug_view);
        self
    }

    fn update_view(view: &mut dyn View) {
        view.update();
        for view in view.subviews_mut() {
            Self::update_view(view.deref_mut());
        }
    }

    #[cfg(not(any(target_os = "ios", target_os = "android")))]
    fn setup_events(&mut self) {
        let mut this = Rglica::from_ref(self);
        self.drawer
            .on_key_pressed
            .subscribe(move |a| this.on_key_pressed(a.0, a.1));

        let mut this = Rglica::from_ref(self);
        self.drawer
            .on_mouse_click
            .subscribe(move |a| this.on_mouse_click(a.0, a.1));

        let mut this = Rglica::from_ref(self);
        self.drawer
            .on_cursor_moved
            .subscribe(move |a| this.on_cursor_moved(a));

        let mut this = Rglica::from_ref(self);
        self.drawer.on_size_changed.subscribe(move |size| {
            this.on_size_changed(size);
        });

        let mut this = Rglica::from_ref(self);
        self.drawer.on_frame_drawn.subscribe(move |_| this.update());
    }

    fn init(&mut self, size: Size) {
        #[cfg(not(any(target_os = "ios", target_os = "android")))]
        self.setup_events();


        
        GLWrapper::enable_blend();
        GLWrapper::set_clear_color(&Color::GRAY);

        self.root_view.calculate_absolute_frame();

        self.set_size(size);

    }
}

#[cfg(not(any(target_os = "ios", target_os = "android")))]
impl Screen {
    fn on_cursor_moved(&mut self, position: Point) {
        self.cursor_position = position;
        self.on_touch(Touch {
            id:       1,
            position: self.cursor_position,
            event:    Event::Moved,
        });
    }

    fn on_mouse_click(&mut self, _button: glfw::MouseButton, state: Action) {
        self.on_touch(Touch {
            id:       1,
            position: self.cursor_position,
            event:    Event::from_state(ButtonState::from_glfw(state)),
        })
    }

    fn on_key_pressed(&mut self, key: Key, action: Action) {
        if action != Action::Press {
            return;
        }
        self.view
            .level_mut()
            .on_key_pressed(key.get_name().unwrap_or_else(|| "unknown".into()))
    }
}

impl Screen {
    fn calculate_fps(&mut self) {
        let now = Utc::now().timestamp_nanos();

        let interval = now - self.prev_time;
        self.prev_time = now;

        self.frame_time = interval as f64 / 1000000000.0;
        self.fps = (1.0 / self.frame_time as f64) as u64;

        if self.debug_view.is_ok() {
            self.debug_view.fps.set(self.fps);
        }
    }

    pub fn update(&mut self) {
        self.calculate_fps();

        GLWrapper::clear();

        self.update_level();

        Screen::update_view(self.root_view.deref_mut());
        self.root_view.calculate_absolute_frame();
        self.ui_drawer.draw(self.root_view.deref_mut());

        self.ui_drawer.reset_viewport();
    }

    fn update_level(&mut self) {
        if self.view.is_null() {
            return;
        }

        let level = self.view.level_mut();

        level.level_mut().update_physics();
        level.update();

        let drawer = self.sprites_drawer.deref();

        drawer.set_camera_position(level.player().position());

        for sprite in level.sprites() {
            drawer.draw(sprite.deref());
        }
    }

    pub fn set_size(&mut self, size: Size) -> &mut Self {
        #[cfg(not(any(target_os = "ios", target_os = "android")))]
        self.drawer.set_size(size);
        self.on_size_changed(size);
        self
    }

    fn on_size_changed(&mut self, size: Size) {
        self.ui_drawer.set_size(size);
        self.root_view.set_frame(size.into());
        self.sprites_drawer.set_resolution(&size);
        self.sprites_drawer.set_camera_position((0, 0).into());
        self.update();
    }

    #[cfg(not(any(target_os = "ios", target_os = "android")))]
    pub fn start_main_loop(&mut self) {
        self.drawer.start_main_loop()
    }
}

impl Screen {
    pub fn new(size: Size) -> Self {
        error!("sokol");
        let mut font_path = ui::DEFAULT_FONT_PATH.lock().unwrap();
        *font_path = paths::fonts().join("SF.otf");
        drop(font_path);
        #[cfg(not(any(target_os = "ios", target_os = "android")))]
        let drawer = GLDrawer::new(size);
        error!("creating assets");
        let assets = Rc::new(Assets::default());
        error!("constructing screen");
        let mut screen = Self {
            #[cfg(not(any(target_os = "ios", target_os = "android")))]
            cursor_position: default(),
            root_view: ViewBase::boxed(),
            debug_view: Default::default(),
            view: default(),
            #[cfg(not(any(target_os = "ios", target_os = "android")))]
            drawer,
            ui_drawer: UIDrawer::new(assets.clone()),
            sprites_drawer: TESpritesDrawer::new(assets),
            fps: Default::default(),
            prev_time: Default::default(),
            frame_time: Default::default(),
        };

        error!("krita");

        screen.init(size);

        error!("ooo");

        screen
    }
}
