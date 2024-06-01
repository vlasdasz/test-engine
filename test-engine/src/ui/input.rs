use gm::Color;
use log::warn;
use ui::{
    check_touch, Container, Touch, TouchStack, UIEvents, UIManager, ViewData, ViewFrame, ViewSetup,
    ViewSubviews,
};
pub use winit::event::KeyEvent;

const LOG_TOUCHES: bool = false;
const DRAW_TOUCHES: bool = false;

pub struct Input;

impl Input {
    pub fn on_char(ch: char) {
        UIManager::keymap().check(ch);
        UIEvents::keyboard_input().trigger(ch);
    }

    pub fn process_touch_event(mut touch: Touch) -> bool {
        UIEvents::on_debug_touch().trigger(touch);

        if UIManager::touch_disabled() {
            return false;
        }

        UIEvents::on_touch().trigger(touch);

        if LOG_TOUCHES && !touch.is_moved() {
            warn!("{touch:?}");
        }

        if (DRAW_TOUCHES || UIManager::draw_touches()) && !touch.is_moved() {
            let mut view = Container::new();
            view.set_z_position(0.1);
            view.set_size((5, 5)).set_color(Color::random());
            view.set_center(touch.position);
            UIManager::root_view_weak().__add_subview_internal(view, true);
        }

        let _level_touch = touch;
        // TODO: Revisit scale
        // if Platform::DESKTOP {
        //     touch.position = self.cursor_position / UIManager::ui_scale();
        // } else {
        //     touch.position /= UIManager::ui_scale();
        // }

        for view in TouchStack::touch_views() {
            if check_touch(view, &mut touch) {
                return true;
            }
        }

        // if let Some(level) = &mut self.level {
        //     level.set_cursor_position(level_touch.position);
        //     if touch.is_began() {
        //         level.add_touch(level_touch.position)
        //     }
        // }

        false
    }
}
