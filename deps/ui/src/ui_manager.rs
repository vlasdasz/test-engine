use std::{ops::Deref, sync::Mutex};

use gm::flat::{Rect, Size};
use refs::{Own, Strong, ToWeak, Weak};
use rtools::Unwrap;
use smart_default::SmartDefault;

use crate::{layout::Placer, view::ViewSubviews, BaseView, UIDrawer, View};

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

    pub(crate) deleted_views: Vec<Own<dyn View>>,

    touch_disabled: bool,

    // #[default = 1.0]
    // ui_scale:     f32,
    #[default = 1.0]
    display_scale: f32,

    window_size: Size,

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
        own.weak()
    }

    pub fn set_window_size(size: impl Into<Size>) {
        Self::get().window_size = size.into();
    }

    pub fn window_size() -> Size {
        Self::get().window_size
    }

    pub fn root_view_size() -> Size {
        Self::window_size() // / UIManager::ui_scale()
    }

    pub fn root_view() -> Weak<dyn View> {
        Self::get().root_view.weak()
    }

    pub fn update() {
        Self::get().deleted_views.clear()
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
        UIManager::root_view().add_subview(view).place.as_background();
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
    /// There are 2 types of scale
    /// Display scale - constant for display on mac and iPhones, always 1 on
    /// other OS (probably) UI scale - adjustable in runtime
    pub fn rescale_frame(rect: &Rect) -> Rect {
        let scale = Self::display_scale();
        let rect = rect; // * UIManager::ui_scale();

        let rect: Rect = (
            rect.origin.x * scale,
            (Self::window_size().height /* UIManager::ui_scale()*/ - rect.origin.y - rect.size.height)
                * scale,
            rect.size.width * scale,
            rect.size.height * scale,
        )
            .into();

        rect
        // (
        //     rect.origin.x,
        //     (UIManager::window_size().height - rect.origin.y -
        // rect.size.height),     rect.size.width,
        //     rect.size.height,
        // )
        //     .into()
    }

    // pub fn ui_scale() -> f32 {
    //     Self::get().ui_scale
    // }
    //
    // pub fn set_ui_scale(scale: f32) {
    //     Self::get().ui_scale = scale;
    //     UIManager::root_view().set_frame(Self::scaled_ui_window_size());
    // }

    #[cfg(windows)]
    /// On windows display scale is always 1. Only ui scale is responsible for
    /// elements size
    pub fn display_scale() -> f32 {
        1.0
    }

    #[cfg(not(windows))]
    pub fn display_scale() -> f32 {
        Self::get().display_scale
    }

    pub fn set_display_scale(scale: f32) {
        Self::get().display_scale = scale
    }
}
