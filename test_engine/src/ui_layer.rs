use std::{ops::Deref, rc::Rc};

#[cfg(desktop)]
use gl_wrapper::events::Events;
#[cfg(desktop)]
use glfw::{Action, Key};
use gm::flat::Point;
use rtools::{platform::Platform, Boxed, Rglica, ToRglica};
use sprites::{Control, Level, Player, SpritesDrawer};
#[cfg(desktop)]
use ui::input::touch::{ButtonState, TouchEvent};
use ui::{Touch, View, ViewBase, ViewFrame, ViewSubviews, ViewTouch};

use crate::{assets::Assets, debug_view::DebugView, main_view::MainView, ui_drawer::TEUIDrawer, Keymap};

pub struct UILayer {
    pub ui_cursor_position: Point,
    pub cursor_position:    Point,
    pub root_view:          Box<dyn View>,
    pub debug_view:         Rglica<DebugView>,
    pub view:               Rglica<dyn MainView>,

    pub sprites_drawer: Rglica<dyn SpritesDrawer>,

    pub keymap: Rc<Keymap>,
    pub drawer: TEUIDrawer,

    #[cfg(desktop)]
    pub events: Rglica<Events>,

    pub fps:        u64,
    pub prev_time:  i64,
    pub frame_time: f64,

    scale: f32,
}

impl UILayer {
    pub fn new(assets: Rc<Assets>, sprites_drawer: Rglica<dyn SpritesDrawer>) -> Box<Self> {
        Box::new(Self {
            ui_cursor_position: Default::default(),
            cursor_position: Default::default(),
            root_view: ViewBase::boxed(),
            debug_view: Default::default(),
            view: Default::default(),
            sprites_drawer,
            keymap: Default::default(),
            drawer: TEUIDrawer::new(assets),
            #[cfg(desktop)]
            events: Default::default(),
            fps: Default::default(),
            prev_time: Default::default(),
            frame_time: Default::default(),
            scale: 1.0,
        })
        .setup_keymap()
    }
}

impl UILayer {
    fn level(&self) -> Rglica<dyn Level> {
        self.view.level().rglica()
    }

    fn player(&self) -> Rglica<Player> {
        self.view.player()
    }

    fn setup_keymap(self: Box<Self>) -> Box<Self> {
        let s = self.deref();

        self.keymap.add("-", s, |s| s.level().multiply_scale(0.8));
        self.keymap.add("=", s, |s| s.level().multiply_scale(1.2));

        self.keymap.add("a", s, |s| s.player().go_left());
        self.keymap.add("d", s, |s| s.player().go_right());
        self.keymap.add("s", s, |s| s.player().go_down());
        self.keymap.add("w", s, |s| s.player().jump());
        self.keymap.add(" ", s, |s| s.player().jump());

        self
    }
}

impl UILayer {
    pub fn on_touch(&mut self, mut touch: Touch) {
        error!("{:?}", touch);
        let level_touch = touch;
        if Platform::DESKTOP {
            touch.position = self.ui_cursor_position;
        } else {
            touch.position /= self.scale;
        }
        if !self.root_view.check_touch(&mut touch) {
            self.view.pass_touch_to_level(level_touch)
        }
    }

    pub fn set_view(&mut self, mut view: Box<dyn MainView>) {
        if self.view.is_ok() {
            self.view.remove_from_superview();
        }
        self.view = view.to_rglica();
        view.set_ui(self.to_rglica());
        view.set_sprites_drawer(self.sprites_drawer);
        self.root_view.add_boxed(view);
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
        self.drawer.set_scale(scale);
        self.root_view.set_frame(self.drawer.window_size / scale);
    }

    pub fn add_debug_view(&mut self) {
        self.debug_view = self.root_view.add_view();
    }
}

#[cfg(desktop)]
impl UILayer {
    fn on_cursor_moved(&mut self, position: Point) {
        self.cursor_position = position;
        self.ui_cursor_position = position / self.scale;
        self.on_touch(Touch {
            id:       1,
            position: self.cursor_position,
            event:    TouchEvent::Moved,
        })
    }

    fn on_mouse_click(&mut self, _button: glfw::MouseButton, state: Action) {
        self.on_touch(Touch {
            id:       1,
            position: self.cursor_position,
            event:    TouchEvent::from_state(ButtonState::from_glfw(state)),
        })
    }

    fn on_key_pressed(&mut self, key: Key, action: Action) {
        if action != Action::Press {
            return;
        }

        let key = key.get_name().unwrap_or_else({
            || {
                match key {
                    Key::Space => " ",
                    _ => "unknown",
                }
                .into()
            }
        });

        self.keymap.check(&key);
        self.view.level_mut().on_key_pressed(&key);
    }

    pub fn setup_events(&mut self) {
        self.events
            .on_key_pressed
            .set(self, |a, this| this.on_key_pressed(a.0, a.1));

        self.events
            .on_mouse_click
            .set(self, |a, this| this.on_mouse_click(a.0, a.1));

        self.events
            .on_cursor_moved
            .set(self, |a, this| this.on_cursor_moved(a))
    }
}
