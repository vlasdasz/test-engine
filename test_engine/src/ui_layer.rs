use std::rc::Rc;

#[cfg(desktop)]
use gl_wrapper::gl_events::GlEvents;
#[cfg(desktop)]
use glfw::{Action, Key};
use gm::flat::{Point, Size};
use rtools::{platform::Platform, IntoF32, ToRglica};
use sprites::Level;
#[cfg(desktop)]
use ui::input::TouchEvent;
use ui::{
    get_ui_drawer,
    input::{ControlButton, KeyEvent, KeyboardButton, UIEvents},
    Touch, View, ViewFrame, ViewLayout, ViewTouch,
};

use crate::Keymap;

pub struct UILayer {
    pub level: Option<Box<dyn Level>>,

    pub ui_cursor_position: Point,
    pub cursor_position:    Point,
    pub view:               Box<dyn View>,

    pub keymap: Rc<Keymap>,

    pub fps:        u64,
    pub prev_time:  i64,
    pub frame_time: f64,

    pub screen_size: Size,

    scale: f32,
}

impl UILayer {
    pub fn on_touch(&mut self, mut touch: Touch) {
        if !touch.is_moved() {
            trace!("{:?}", touch);
        }
        let level_touch = touch;
        if Platform::DESKTOP {
            touch.position = self.ui_cursor_position;
        } else {
            touch.position /= self.scale;
        }
        if !self.view.check_touch(&mut touch) {
            if let Some(level) = &mut self.level {
                level.set_cursor_position(level_touch.position);
                if touch.is_began() {
                    level.add_touch(level_touch.position)
                }
            }
        }
        get_ui_drawer().remove_scheduled();
    }

    pub fn set_view(&mut self, view: Box<dyn View>) {
        get_ui_drawer().set_next_view(view);
    }

    pub(crate) fn init_next_view(&mut self) {
        let Some(view) = get_ui_drawer().next_view() else {
            return;
        };
        get_ui_drawer().set_root_view(view.to_rglica());
        self.view = view;
        self.view.set_size(self.screen_size);
        self.view.init_views();
        self.view.setup();
        self.view.calculate_frames();
    }

    pub fn set_level(&mut self, level: Box<dyn Level>) {
        self.level = level.into();
        self.level.as_mut().unwrap().setup();
    }

    pub fn scale(&self) -> f32 {
        self.scale
    }

    pub fn set_scale(&mut self, scale: impl IntoF32) {
        self.scale = scale.into_f32();
        get_ui_drawer().set_scale(self.scale);
        self.view.set_frame(*get_ui_drawer().window_size() / self.scale);
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
            event:    ui::input::MouseButtonState::from_glfw(state).into(),
        })
    }

    fn on_key_pressed(&mut self, key: char) {
        self.keymap.check(key);
        if let Some(level) = &mut self.level {
            level.on_key_pressed(key)
        }
    }

    pub fn setup_events(&mut self) {
        let ev = GlEvents::get();

        ev.key_pressed.set(self, |this, a| {
            let key = a.0;
            let action = a.1;

            let button = match key {
                Key::Space => KeyboardButton::Letter(' '),
                Key::LeftControl | Key::RightControl => ControlButton::Ctrl.into(),
                Key::LeftAlt | Key::RightAlt => ControlButton::Alt.into(),
                Key::Delete => ControlButton::Del.into(),
                Key::Escape => ControlButton::Escape.into(),
                Key::Backspace => ControlButton::Backspace.into(),
                _ => match key.get_name() {
                    Some(name) => name.chars().next().unwrap().into(),
                    None => ControlButton::Unknown.into(),
                },
            };

            let event = KeyEvent {
                button,
                state: action.into(),
            };

            if let Some(char) = event.char() && event.is_press() {
                this.on_key_pressed(char);
            }

            if !event.is_release() {
                UIEvents::get().key_pressed.trigger(event);
            }
        });

        ev.mouse_click.set(self, |this, a| this.on_mouse_click(a.0, a.1));

        ev.cursor_moved.set(self, |this, a| this.on_cursor_moved(a))
    }
}

impl UILayer {
    pub fn new(size: impl Into<Size>, view: Box<dyn View>) -> Box<Self> {
        Box::new(Self {
            level: Default::default(),

            ui_cursor_position: Default::default(),
            cursor_position: Default::default(),
            view,

            keymap: Default::default(),

            fps: Default::default(),
            prev_time: Default::default(),
            frame_time: Default::default(),

            screen_size: size.into(),

            scale: 1.0,
        })
    }
}
