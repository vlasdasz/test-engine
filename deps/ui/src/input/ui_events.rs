use refs::{is_main_thread, Weak};
use vents::Event;

use crate::{input::keyboard::KeyEvent, View, ViewTouch};

static mut EVENTS: *mut UIEvents = std::ptr::null_mut();

pub struct UIEvents {
    pub key_pressed:   Event<KeyEvent>,
    pub scroll:        Event<f32>,
    pub selected_view: Weak<dyn View>,
}

impl UIEvents {
    fn init() -> Self {
        Self {
            key_pressed:   Default::default(),
            scroll:        Default::default(),
            selected_view: Default::default(),
        }
    }

    pub fn get() -> &'static mut Self {
        debug_assert!(is_main_thread());
        unsafe {
            if EVENTS.is_null() {
                EVENTS = Box::into_raw(Box::new(Self::init()));
            }
            EVENTS.as_mut().unwrap()
        }
    }
}

impl UIEvents {
    pub fn unselect_view(&mut self) {
        if let Some(view) = self.selected_view.get() {
            view.set_selected(false)
        }
    }
}
