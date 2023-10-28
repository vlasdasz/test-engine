//! Represents UI elements

use std::rc::Rc;

use gm::flat::Point;
use sprites::Level;
use ui::{
    check_touch,
    input::UIEvents,
    refs::{Own, Weak},
    Touch, TouchStack, UIManager,
};
use ui_views::debug_view::DebugView;

use crate::{Keymap, Screen};

const LOG_TOUCHES: bool = true;

#[derive(Default)]
pub struct UILayer {
    pub level: Option<Own<dyn Level>>,

    pub cursor_position: Point,

    pub(crate) keymap: Rc<Keymap>,

    pub fps:        u64,
    pub prev_time:  i64,
    pub frame_time: f32,

    pub debug_view: Weak<DebugView>,

    #[cfg(desktop)]
    pub(crate) shift_pressed: bool,
}

impl UILayer {
    pub fn on_touch(&mut self, mut touch: Touch) {
        if UIManager::touch_disabled() {
            return;
        }

        UIEvents::get().on_touch.trigger(touch);

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

    pub fn keymap() -> Rc<Keymap> {
        Screen::current().ui.keymap.clone()
    }
}
