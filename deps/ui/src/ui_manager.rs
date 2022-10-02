use std::ops::{Deref, DerefMut};

use gm::flat::Size;
use rtools::{static_default, Strong, UnwrapBox, Weak};
use smart_default::SmartDefault;

use crate::{layout::Placer, view::ViewSubviews, BaseView, UIAnimation, UIDrawer, View};

#[derive(SmartDefault)]
pub struct UIManager {
    drawer: UnwrapBox<dyn UIDrawer>,

    #[default({
        let mut view = Strong::<BaseView>::default();
        view.place = Placer::new(view.weak_view()).into();
        view
    })]
    root_view: Strong<dyn View>,

    next_view: Option<Strong<dyn View>>,

    different_touch_root: Weak<dyn View>,

    pub(crate) animations: Vec<UIAnimation>,

    views_to_remove: Vec<Weak<dyn View>>,
    touch_disabled:  bool,

    #[default = 1.0]
    scale:        f32,
    #[default = 1.0]
    screen_scale: f32,

    pub open_keyboard:  bool,
    pub close_keyboard: bool,
}
static_default!(UIManager);

impl UIManager {
    pub fn window_size() -> Size {
        Self::get().root_view.frame.size
    }

    pub fn root_view() -> &'static mut dyn View {
        Self::get().root_view.deref_mut()
    }

    pub(crate) fn views_to_remove() -> &'static mut Vec<Weak<dyn View>> {
        &mut Self::get().views_to_remove
    }

    pub(crate) fn animations() -> &'static [UIAnimation] {
        &Self::get().animations
    }

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

    pub fn touch_root() -> &'static mut dyn View {
        if Self::get().different_touch_root.is_ok() {
            Self::get().different_touch_root.deref_mut()
        } else {
            Self::get().root_view.deref_mut()
        }
    }
}

impl UIManager {
    pub(crate) fn schedule_remove(mut view: Weak<dyn View>) {
        view.is_deleted = true;
        UIManager::views_to_remove().push(view);
    }

    pub fn remove_scheduled() {
        if UIManager::views_to_remove().is_empty() {
            return;
        }
        let to_remove = UIManager::views_to_remove().drain(..);
        for view in to_remove {
            let index = view
                .superview()
                .subviews()
                .iter()
                .position(|sub| view.addr() == sub.addr())
                .unwrap();
            view.superview().remove_subview_at(index);
        }
    }

    pub fn set_scheduled() {
        let Some(mut view) = UIManager::get().next_view.take() else {
            return;
        };
        UIManager::root_view().remove_all_subviews();
        view.frame = UIManager::root_view().frame;
        let mut view = UIManager::root_view().add_subview(view);
        view.place.as_background();
    }

    pub fn set_view(view: Strong<dyn View>) {
        UIManager::get().next_view.replace(view);
    }
}

impl UIManager {
    pub fn drawer() -> &'static dyn UIDrawer {
        Self::get().drawer.deref()
    }

    pub fn set_drawer(drawer: Strong<dyn UIDrawer>) {
        Self::get().drawer = UnwrapBox::from_box(drawer)
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
