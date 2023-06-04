use std::sync::{Mutex, MutexGuard, OnceLock};

use gm::flat::{Point, Rect, Size};
use refs::{Own, ToWeak, Weak};
use smart_default::SmartDefault;

use crate::{layout::Placer, view::ViewSubviews, Container, UIDrawer, UIEvent, View};

static UI_MANAGER: OnceLock<Mutex<UIManager>> = OnceLock::new();
static DRAWER: OnceLock<Mutex<Box<dyn UIDrawer>>> = OnceLock::new();

#[derive(SmartDefault)]
pub struct UIManager {
    #[default({
        let mut view = Own::<Container>::default();
        view.place = Placer::new(view.weak_view()).into();
        view
    })]
    root_view: Own<dyn View>,

    next_view: Option<Own<dyn View>>,

    pub(crate) deleted_views: Vec<Own<dyn View>>,

    touch_disabled: bool,

    // #[default = 1.0]
    // ui_scale:     f32,
    #[default = 1.0]
    display_scale: f32,

    window_size: Size,

    pub touch_views: Vec<Weak<dyn View>>,

    pub on_scroll: UIEvent<Point>,

    pub open_keyboard:  bool,
    pub close_keyboard: bool,
}

impl UIManager {
    pub fn drop() {
        *Self::get() = UIManager::default();
    }

    pub fn get() -> MutexGuard<'static, Self> {
        UI_MANAGER.get_or_init(|| UIManager::default().into()).lock().unwrap()
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

    pub fn enable_touch_for(view: Weak<dyn View>) {
        let mut this = Self::get();
        this.touch_views.retain(|a| !a.freed());
        this.touch_views.push(view);
    }

    pub fn disable_touch_for(view: Weak<dyn View>) {
        Self::get().touch_views.retain(|a| a.addr() != view.addr());
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
    pub fn drawer() -> MutexGuard<'static, Box<dyn UIDrawer>> {
        DRAWER.get().unwrap().lock().unwrap()
    }

    pub fn set_drawer(drawer: impl UIDrawer + 'static) {
        DRAWER.set(Mutex::new(Box::new(drawer))).unwrap();
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
