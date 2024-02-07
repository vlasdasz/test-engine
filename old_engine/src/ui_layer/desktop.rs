#![cfg(desktop)]

use gm::flat::Point;
use ui::{input::TouchEvent, Touch};
use wgpu_wrapper::MouseButton;

use crate::ui_layer::UILayer;

impl UILayer {
    fn _on_cursor_moved(&mut self, position: Point) {
        self.cursor_position = position;
        self.on_touch(Touch {
            id:       1,
            position: self.cursor_position,
            event:    TouchEvent::Moved,
            button:   MouseButton::Left,
        })
    }

    // fn on_mouse_click(&mut self, _button: glfw::MouseButton, state: Action) {
    //     self.on_touch(Touch {
    //         id:       1,
    //         position: self.cursor_position,
    //         event:    ui::input::MouseButtonState::from_glfw(state).into(),
    //     })
    // }

    fn _on_key_pressed(&mut self, key: char) {
        self.keymap.check(key);
    }

    pub fn setup_events(&mut self) {
        // let ev = SystemEvents::get();
        //
        // let mut this = ui::refs::weak_from_ref(self);
        // ev.key_pressed.val(move |a| {
        //     let key = a.0;
        //     let action = a.1;
        //
        //     let button = match key {
        //         Key::Space => KeyboardButton::Letter(' '),
        //         Key::LeftControl | Key::RightControl =>
        // ControlButton::Ctrl.into(),         Key::LeftAlt |
        // Key::RightAlt => ControlButton::Alt.into(),
        //         Key::Delete => ControlButton::Del.into(),
        //         Key::LeftShift | Key::RightShift =>
        // ControlButton::Shift.into(),         Key::Escape =>
        // ControlButton::Escape.into(),         Key::Backspace =>
        // ControlButton::Backspace.into(),         _ => match
        // key.get_name() {             Some(name) =>
        // name.chars().next().unwrap().into(),             None =>
        // ControlButton::Unknown.into(),         },
        //     };
        //
        //     let mut event = KeyEvent {
        //         button,
        //         state: action.into(),
        //     };
        //
        //     if event.is_control(ControlButton::Shift) {
        //         this.shift_pressed = event.is_press();
        //     }
        //
        //     if this.shift_pressed {
        //         event.uppercase();
        //     }
        //
        //     if let Some(char) = event.char()
        //         && event.is_press()
        //     {
        //         this.on_key_pressed(char);
        //     }
        //
        //     if !event.is_release() {
        //         UIEvents::get().key_pressed.trigger(event);
        //     }
        // });
        //
        // ev.mouse_click.val(move |a| this.on_mouse_click(a.0, a.1));
        // ev.cursor_moved.val(move |a| this.on_cursor_moved(a));
        //
        // ev.scroll.val(UIManager::trigger_scroll);
        // ev.file_drop.val(UIManager::trigger_drop_file);
    }
}
