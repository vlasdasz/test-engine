use gm::{flat::Size, volume::GyroData};
use refs::MainLock;
use vents::Event;

use crate::{Touch, UIEvent};

static UI_EVENTS: MainLock<UIEvents> = MainLock::new();

#[derive(Default)]
pub struct UIEvents {
    on_touch:       Event<Touch>,
    on_debug_touch: Event<Touch>,
    size_changed:   UIEvent<Size<u32>>,
    gyro:           UIEvent<GyroData>,
    keyboard_input: UIEvent<char>,
}

impl UIEvents {
    pub fn on_touch() -> &'static Event<Touch> {
        &UI_EVENTS.on_touch
    }

    /// Is never disabled
    pub fn on_debug_touch() -> &'static Event<Touch> {
        &UI_EVENTS.on_debug_touch
    }

    pub fn size_changed() -> &'static UIEvent<Size<u32>> {
        &UI_EVENTS.size_changed
    }

    pub fn keyboard_input() -> &'static UIEvent<char> {
        &UI_EVENTS.keyboard_input
    }

    pub fn gyro() -> &'static UIEvent<GyroData> {
        &UI_EVENTS.gyro
    }
}
