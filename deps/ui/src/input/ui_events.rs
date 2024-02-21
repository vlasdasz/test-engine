use gm::flat::Size;
use refs::MainLock;
use vents::Event;

use crate::Touch;

static UI_EVENTS: MainLock<UIEvents> = MainLock::new();

#[derive(Default)]
pub struct UIEvents {
    on_touch:       Event<Touch>,
    size_changed:   Event<Size<u32>>,
    keyboard_input: Event<char>,
}

impl UIEvents {
    pub(crate) fn get() -> &'static Self {
        &UI_EVENTS
    }

    pub fn on_touch() -> &'static Event<Touch> {
        &Self::get().on_touch
    }

    pub fn size_changed() -> &'static Event<Size<u32>> {
        &Self::get().size_changed
    }

    pub fn keyboard_input() -> &'static Event<char> {
        &Self::get().keyboard_input
    }
}
