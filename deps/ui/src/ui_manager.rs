use std::{ops::Deref, sync::Mutex};

use gm::flat::Size;
use refs::{Own, Strong, ToWeak, Weak};
use rtools::Unwrap;
use smart_default::SmartDefault;

use crate::{
    layout::Placer,
    view::{ViewFrame, ViewSubviews},
    BaseView, UIAnimation, UIDrawer, View,
};

static MANAGER: Mutex<Unwrap<Own<UIManager>>> = Mutex::new(Unwrap::default());

#[derive(SmartDefault)]
pub struct UIManager {
    drawer: Unwrap<Own<dyn UIDrawer>>,

    #[default({
        let mut view = Strong::<BaseView>::default();
        view.place = Placer::new(view.weak_view()).into();
        view
    })]
    root_view: Strong<dyn View>,

    next_view: Option<Own<dyn View>>,

    pub(crate) touch_stack: Vec<Weak<dyn View>>,

    pub(crate) animations: Vec<UIAnimation>,

    touch_disabled: bool,

    #[default = 1.0]
    scale:        f32,
    #[default = 1.0]
    screen_scale: f32,

    pub open_keyboard:  bool,
    pub close_keyboard: bool,
}

impl UIManager {
    pub fn drop() {
        MANAGER.lock().unwrap().take();
    }

    pub fn get() -> Weak<UIManager> {
        let mut lock = MANAGER.lock().unwrap();

        if lock.is_none() {
            *lock = Unwrap::new(Own::new(Self::default()));
        }

        let unwrap = lock.deref();
        let own = unwrap.deref();
        let weak = own.weak();
        weak
    }

    pub fn window_size() -> Size {
        Self::get().root_view.frame.size
    }

    pub fn root_view() -> Weak<dyn View> {
        Self::get().root_view.weak()
    }

    // pub(crate) fn views_to_remove() -> &'static mut Vec<Weak<dyn View>> {
    //     &mut Self::get().views_to_remove
    // }

    // pub(crate) fn animations() -> &'static [UIAnimation] {
    //     &Self::get().animations
    // }

    pub(crate) fn add_animation(anim: UIAnimation) {
        Self::get().animations.push(anim)
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

    pub fn touch_root() -> Weak<dyn View> {
        let mut this = Self::get();

        this.touch_stack.retain(|a| !a.freed());

        if let Some(touch) = this.touch_stack.last_mut() {
            *touch
        } else {
            this.root_view.weak()
        }
    }
}

impl UIManager {
    pub fn set_scheduled() {
        let Some(mut view) = UIManager::get().next_view.take() else {
            return;
        };
        UIManager::root_view().remove_all_subviews();
        view.frame = UIManager::root_view().frame;
        let mut view = UIManager::root_view().add_subview(view);
        view.place.as_background();
    }

    pub fn set_view(view: Own<dyn View>) {
        UIManager::get().next_view.replace(view);
    }
}

impl UIManager {
    pub fn drawer() -> Weak<dyn UIDrawer> {
        Self::get().drawer.deref().weak()
    }

    pub fn set_drawer(drawer: Own<dyn UIDrawer>) {
        Self::get().drawer = Unwrap::from(drawer)
    }
}

impl UIManager {
    pub fn scale() -> f32 {
        Self::get().scale
    }

    pub fn set_scale(scale: f32) {
        Self::get().scale = scale;
    }

    pub fn screen_scale() -> f32 {
        Self::get().screen_scale
    }

    pub fn screen_size() -> Size {
        Self::root_view().size()
    }

    #[cfg(any(windows, linux, freebsd))]
    pub fn set_screen_scale(_scale: f32) {
        Self::get().screen_scale = 1.0
    }

    #[cfg(macos)]
    pub fn set_screen_scale(scale: f32) {
        Self::get().screen_scale = scale
    }

    #[cfg(mobile)]
    pub fn set_screen_scale(scale: f32) {
        Self::get().screen_scale = scale
    }
}
