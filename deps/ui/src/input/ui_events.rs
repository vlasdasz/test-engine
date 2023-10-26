use std::cell::RefCell;

use gm::volume::GyroData;
use refs::{MainLock, Weak};
use vents::Event;

use crate::{input::keyboard::KeyEvent, Touch, View};

static UI_EVENTS: MainLock<UIEvents> = MainLock::new();

#[derive(Default)]
pub struct UIEvents {
    pub key_pressed:  Event<KeyEvent>,
    pub scroll:       Event<f32>,
    pub gyro_changed: Event<GyroData>,
    pub on_touch:     Event<Touch>,
    selected_view:    RefCell<Weak<dyn View>>,
}

impl UIEvents {
    pub fn get() -> &'static Self {
        &UI_EVENTS
    }
}

impl UIEvents {
    pub fn unselect_view(&self) {
        let mut selected_view = self.selected_view.borrow_mut();
        if !selected_view.is_ok() {
            return;
        }
        selected_view.is_selected = false;
        selected_view.on_selection_changed(false);
        *selected_view = Default::default();
    }

    pub fn set_selected(&self, mut view: Weak<dyn View>, selected: bool) {
        let mut selected_view = self.selected_view.borrow_mut();

        if let Some(selected) = selected_view.get() {
            selected.is_selected = false;
            selected.on_selection_changed(false);
            *selected_view = Default::default();
        }

        if selected {
            *selected_view = view;
        }

        view.is_selected = selected;
        view.on_selection_changed(selected);
    }
}
