use gm::color::Color;
use level::LevelManager;
use log::warn;
use ui::{Container, Setup, Touch, TouchStack, UIEvents, UIManager, ViewData, ViewFrame, check_touch};
pub use winit::{event::KeyEvent, keyboard::NamedKey};

const LOG_TOUCHES: bool = false;
const DRAW_TOUCHES: bool = false;

pub struct Input;

impl Input {
    pub fn on_char(ch: char) {
        UIManager::keymap().check(ch);
        UIEvents::keyboard_input().trigger(ch);
    }

    pub fn on_key(key: NamedKey) {
        UIEvents::keyboard_key().trigger(key);
    }

    pub fn process_touch_event(mut touch: Touch) -> bool {
        UIEvents::on_debug_touch().trigger(touch);

        if UIManager::touch_disabled() {
            return false;
        }

        let original_pos = touch.position;

        touch.position *= 1.0 / UIManager::scale();

        UIManager::set_cursor_position(touch.position);
        UIEvents::on_touch().trigger(touch);

        if LOG_TOUCHES && !touch.is_moved() {
            warn!("{touch:?}");
        }

        if (DRAW_TOUCHES || UIManager::draw_touches()) && !touch.is_moved() {
            let mut view = Container::new();
            view.set_z_position(0.1);
            view.set_size(5, 5).set_color(Color::random());
            view.set_center(touch.position);
            UIManager::root_view().add_subview_to_root(view);
        }

        for view in TouchStack::touch_views() {
            if check_touch(view, &mut touch) {
                return true;
            }
        }

        if touch.is_began() && !LevelManager::no_level() {
            return LevelManager::level_weak().add_touch(original_pos);
        }

        false
    }
}
