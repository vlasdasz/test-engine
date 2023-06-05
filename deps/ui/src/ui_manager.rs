use std::{
    ops::Deref,
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering},
        Mutex, MutexGuard, OnceLock,
    },
};

use gm::flat::{Point, Rect, Size};
use nonempty::NonEmpty;
use refs::{Own, ToWeak, Weak};

use crate::{
    layout::Placer, touch_layer::TouchLayer, view::ViewSubviews, Container, UIDrawer, UIEvent, View,
};

static UI_MANAGER: OnceLock<UIManager> = OnceLock::new();
static DRAWER: OnceLock<Mutex<Box<dyn UIDrawer>>> = OnceLock::new();

pub struct UIManager {
    root_view: Own<dyn View>,

    next_view: Mutex<Option<Own<dyn View>>>,

    pub(crate) deleted_views: Mutex<Vec<Own<dyn View>>>,

    touch_disabled: AtomicBool,

    display_scale: Mutex<f32>,

    window_size: Mutex<Size>,

    pub(crate) touch_stack: Mutex<NonEmpty<TouchLayer>>,

    on_scroll:    UIEvent<Point>,
    on_drop_file: UIEvent<Vec<PathBuf>>,

    pub open_keyboard:  AtomicBool,
    pub close_keyboard: AtomicBool,
}

impl UIManager {
    fn init() -> Self {
        let mut root_view = Own::<Container>::default();
        let weak_root = root_view.weak_view();
        root_view.place = Placer::new(weak_root).into();

        Self {
            root_view,
            next_view: None.into(),
            deleted_views: Default::default(),
            touch_disabled: false.into(),
            display_scale: 1.0.into(),
            window_size: Default::default(),
            touch_stack: NonEmpty::new(weak_root.into()).into(),
            on_scroll: Default::default(),
            on_drop_file: Default::default(),
            open_keyboard: false.into(),
            close_keyboard: false.into(),
        }
    }

    pub fn get() -> &'static Self {
        UI_MANAGER.get_or_init(Self::init)
    }

    pub fn set_window_size(size: impl Into<Size>) {
        *Self::get().window_size.lock().unwrap() = size.into();
    }

    pub fn window_size() -> Size {
        *Self::get().window_size.lock().unwrap()
    }

    pub fn root_view_size() -> Size {
        Self::window_size() // / UIManager::ui_scale()
    }

    pub fn root_view() -> Weak<dyn View> {
        Self::get().root_view.weak()
    }

    pub fn update() {
        Self::get().deleted_views.lock().unwrap().clear()
    }
}

impl UIManager {
    pub fn touch_views() -> Vec<Weak<dyn View>> {
        Self::get().touch_stack.lock().unwrap().last().views()
    }

    pub fn touch_disabled() -> bool {
        Self::get().touch_disabled.load(Ordering::Relaxed)
    }

    pub fn disable_touch() {
        Self::get().touch_disabled.store(true, Ordering::Relaxed)
    }

    pub fn enable_touch() {
        Self::get().touch_disabled.store(false, Ordering::Relaxed)
    }

    pub fn enable_touch_for(view: Weak<dyn View>) {
        Self::get().touch_stack.lock().unwrap().last_mut().add(view)
    }

    pub fn disable_touch_for(view: Weak<dyn View>) {
        Self::get().touch_stack.lock().unwrap().last_mut().remove(view)
    }

    pub fn push_touch_layer(view: Weak<dyn View>) {
        Self::get().touch_stack.lock().unwrap().push(view.into())
    }

    pub fn pop_touch_layer(view: Weak<dyn View>) {
        let pop = Self::get().touch_stack.lock().unwrap().pop().unwrap();
        assert_eq!(
            pop.root_addr(),
            view.addr(),
            "Inconsistent pop_touch_view call. Expected: {} got: {}",
            pop.root_name(),
            view.label
        );
    }

    pub fn touch_root_name() -> String {
        Self::get().touch_stack.lock().unwrap().last().root_name()
    }
}

impl UIManager {
    pub fn set_scheduled() {
        let Some(mut view) = UIManager::get().next_view.lock().unwrap().take() else {
            return;
        };
        UIManager::root_view().remove_all_subviews();
        view.frame = UIManager::root_view().frame;
        UIManager::root_view().add_subview(view).place.as_background();
    }

    // TODO: Rework this
    pub fn set_view(view: Own<dyn View>) {
        UIManager::get().next_view.lock().unwrap().replace(view);
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
        *Self::get().display_scale.lock().unwrap()
    }

    pub fn set_display_scale(scale: f32) {
        *Self::get().display_scale.lock().unwrap() = scale
    }
}

impl UIManager {
    pub fn trigger_scroll(scroll: Point) {
        Self::get().on_scroll.trigger(scroll * 10)
    }

    pub fn on_scroll(
        view: impl Deref<Target = impl View + ?Sized>,
        action: impl FnMut(Point) + Send + 'static,
    ) {
        Self::get().on_scroll.val(view, action)
    }

    pub fn trigger_drop_file(files: Vec<PathBuf>) {
        Self::get().on_drop_file.trigger(files)
    }

    pub fn on_drop_file(
        view: impl Deref<Target = impl View + ?Sized>,
        action: impl FnMut(Vec<PathBuf>) + Send + 'static,
    ) {
        Self::get().on_drop_file.val(view, action)
    }
}
