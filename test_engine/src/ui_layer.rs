use std::rc::Rc;

#[cfg(desktop)]
use gl_wrapper::events::Events;
#[cfg(desktop)]
use glfw::{Action, Key};
use gm::flat::Point;
use rtools::{platform::Platform, Boxed, Rglica, ToRglica};
use sprites::SpritesDrawer;
#[cfg(desktop)]
use ui::input::touch::{ButtonState, TouchEvent};
use ui::{view_base::ViewBase, Touch, View, ViewTemplates};

use crate::{assets::Assets, debug_view::DebugView, game_view::GameView, ui_drawer::UIDrawer};

pub struct UILayer {
    pub ui_cursor_position: Point,
    pub cursor_position:    Point,
    pub root_view:          Box<dyn View>,
    pub debug_view:         Rglica<DebugView>,
    pub view:               Rglica<dyn GameView>,

    pub sprites_drawer: Rglica<dyn SpritesDrawer>,

    pub drawer: UIDrawer,

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
            drawer: UIDrawer::new(assets),
            #[cfg(desktop)]
            events: Default::default(),
            fps: Default::default(),
            prev_time: Default::default(),
            frame_time: Default::default(),
            scale: 1.0,
        })
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

    pub fn set_view(&mut self, mut view: Box<dyn GameView>) {
        view.set_sprites_drawer(self.sprites_drawer);
        if self.view.is_ok() {
            self.view.remove_from_superview();
        }
        self.view = view.to_rglica();
        let ui = self.to_rglica();
        self.view.set_ui(ui);
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

        self.view
            .level_mut()
            .on_key_pressed(key.get_name().unwrap_or_else({
                || {
                    match key {
                        Key::Space => " ",
                        _ => "unknown",
                    }
                    .into()
                }
            }))
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
