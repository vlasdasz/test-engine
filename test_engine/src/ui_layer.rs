//! Represents UI elements

use std::rc::Rc;

#[cfg(desktop)]
use gl_wrapper::system_events::SystemEvents;
#[cfg(desktop)]
use glfw::{Action, Key};
use gm::flat::Point;
use smart_default::SmartDefault;
use sprites::Level;
#[cfg(desktop)]
use ui::input::{ControlButton, KeyEvent, KeyboardButton, TouchEvent, UIEvents};
use ui::{
    check_touch,
    refs::{Own, Weak},
    Touch, TouchStack, UIManager,
};
use ui_views::debug_view::DebugView;

use crate::Keymap;

const LOG_TOUCHES: bool = true;

#[derive(SmartDefault)]
pub struct UILayer {
    pub level: Option<Own<dyn Level>>,

    pub cursor_position: Point,

    pub keymap: Rc<Keymap>,

    pub fps:        u64,
    pub prev_time:  i64,
    pub frame_time: f32,

    pub debug_view: Weak<DebugView>,

    #[cfg(desktop)]
    shift_pressed: bool,
}

impl UILayer {
    pub fn on_touch(&mut self, mut touch: Touch) {
        if UIManager::touch_disabled() {
            return;
        }

        if LOG_TOUCHES && !touch.is_moved() {
            warn!("{touch:?}");
        }

        let level_touch = touch;
        // TODO: Revisit scale
        // if Platform::DESKTOP {
        //     touch.position = self.cursor_position / UIManager::ui_scale();
        // } else {
        //     touch.position /= UIManager::ui_scale();
        // }

        for view in TouchStack::touch_views() {
            if check_touch(view, &mut touch) {
                return;
            }
        }

        if let Some(level) = &mut self.level {
            level.set_cursor_position(level_touch.position);
            if touch.is_began() {
                level.add_touch(level_touch.position)
            }
        }
    }

    pub fn set_level(&mut self, level: Own<dyn Level>) {
        self.level = level.into();
        self.level.as_mut().unwrap().setup();
    }
}

#[cfg(desktop)]
impl UILayer {
    fn on_cursor_moved(&mut self, position: Point) {
        self.cursor_position = position;
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
        use ui::refs::ToWeak;

        let ev = SystemEvents::get();

        let mut this = self.weak();
        ev.key_pressed.val(move |a| {
            let key = a.0;
            let action = a.1;

            let button = match key {
                Key::Space => KeyboardButton::Letter(' '),
                Key::LeftControl | Key::RightControl => ControlButton::Ctrl.into(),
                Key::LeftAlt | Key::RightAlt => ControlButton::Alt.into(),
                Key::Delete => ControlButton::Del.into(),
                Key::LeftShift | Key::RightShift => ControlButton::Shift.into(),
                Key::Escape => ControlButton::Escape.into(),
                Key::Backspace => ControlButton::Backspace.into(),
                _ => match key.get_name() {
                    Some(name) => name.chars().next().unwrap().into(),
                    None => ControlButton::Unknown.into(),
                },
            };

            let mut event = KeyEvent {
                button,
                state: action.into(),
            };

            if event.is_control(ControlButton::Shift) {
                this.shift_pressed = event.is_press();
            }

            if this.shift_pressed {
                event.uppercase();
            }

            if let Some(char) = event.char() && event.is_press() {
                this.on_key_pressed(char);
            }

            if !event.is_release() {
                UIEvents::get().key_pressed.trigger(event);
            }
        });

        ev.mouse_click.val(move |a| this.on_mouse_click(a.0, a.1));
        ev.cursor_moved.val(move |a| this.on_cursor_moved(a));

        ev.scroll.val(UIManager::trigger_scroll);
        ev.file_drop.val(UIManager::trigger_drop_file);
    }
}
