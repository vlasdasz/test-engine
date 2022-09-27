use rtools::{static_default, Rglica};
use smart_default::SmartDefault;

use crate::View;

#[derive(SmartDefault)]
pub struct UIManager {
    views_to_remove: Vec<Rglica<dyn View>>,
    touch_disabled:  bool,
}
static_default!(UIManager);

impl UIManager {
    pub fn views_to_remove() -> &'static mut Vec<Rglica<dyn View>> {
        &mut Self::get().views_to_remove
    }
}

impl UIManager {
    pub fn touch_disabled() -> bool {
        Self::get().touch_disabled
    }

    pub fn disable_touch() {
        Self::get().touch_disabled = true
    }

    pub fn enable_touch() {
        Self::get().touch_disabled = false
    }
}
