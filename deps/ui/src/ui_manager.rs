use std::{
    ops::Deref,
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering},
        Mutex, MutexGuard, OnceLock,
    },
};

use gl_wrapper::GLWrapper;
use gm::flat::{IntSize, Point, Rect, Size};
use refs::{Own, Weak};

use crate::{layout::Placer, Container, UIDrawer, UIEvent, View};

static UI_MANAGER: OnceLock<UIManager> = OnceLock::new();
static DRAWER: OnceLock<Mutex<Box<dyn UIDrawer>>> = OnceLock::new();

pub struct UIManager {
    pub(crate) root_view: Own<dyn View>,

    pub(crate) deleted_views: Mutex<Vec<Own<dyn View>>>,

    touch_disabled: AtomicBool,

    display_scale: Mutex<f32>,

    window_size: Mutex<IntSize>,

    on_scroll:    UIEvent<Point>,
    on_drop_file: UIEvent<Vec<PathBuf>>,

    pub open_keyboard:  AtomicBool,
    pub close_keyboard: AtomicBool,
}

impl UIManager {
    fn init() -> Self {
        let mut root_view = Own::<Container>::default();
        root_view.label = "Root view".to_string();
        let weak_root = root_view.weak_view();
        root_view.place = Placer::new(weak_root);

        Self {
            root_view,
            deleted_views: Default::default(),
            touch_disabled: false.into(),
            display_scale: 1.0.into(),
            window_size: Default::default(),
            on_scroll: Default::default(),
            on_drop_file: Default::default(),
            open_keyboard: false.into(),
            close_keyboard: false.into(),
        }
    }

    pub fn get() -> &'static Self {
        UI_MANAGER.get_or_init(Self::init)
    }

    pub fn set_window_size(size: IntSize) {
        *Self::get().window_size.lock().unwrap() = size;
    }

    pub fn window_size() -> IntSize {
        *Self::get().window_size.lock().unwrap()
    }

    pub fn root_view_size() -> Size {
        Self::window_size().into() // / UIManager::ui_scale()
    }

    pub fn root_view() -> Weak<dyn View> {
        Self::get().root_view.weak()
    }

    pub fn update() {
        Self::get().deleted_views.lock().unwrap().clear()
    }
}

impl UIManager {
    pub fn touch_disabled() -> bool {
        Self::get().touch_disabled.load(Ordering::Relaxed)
    }

    pub fn disable_touch() {
        Self::get().touch_disabled.store(true, Ordering::Relaxed)
    }

    pub fn enable_touch() {
        Self::get().touch_disabled.store(false, Ordering::Relaxed)
    }
}

impl UIManager {
    pub fn drawer() -> MutexGuard<'static, Box<dyn UIDrawer>> {
        DRAWER.get().unwrap().lock().unwrap()
    }

    pub fn set_drawer(drawer: impl UIDrawer + 'static) {
        DRAWER.set(Mutex::new(Box::new(drawer))).unwrap();
    }

    pub fn reset_viewport() {
        let window_size = UIManager::window_size();
        let display_scale = UIManager::display_scale();
        GLWrapper::set_viewport((
            0,
            0,
            window_size.width as f32 * display_scale,
            window_size.height as f32 * display_scale,
        ));
    }
}

impl UIManager {
    /// There are 2 types of scale
    /// Display scale - constant for display on mac and iPhones, always 1 on
    /// other OS (probably) UI scale - adjustable in runtime
    pub fn rescale_frame(rect: &Rect) -> Rect {
        let scale = Self::display_scale();
        // let rect = rect * UIManager::ui_scale();

        let rect: Rect = (
            rect.origin.x * scale,
            (Self::window_size().height as f32 /* UIManager::ui_scale()*/ - rect.origin.y - rect.size.height)
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
    //
    #[cfg(not(any(macos, ios)))]
    /// On windows and linux display scale is always 1. Only ui scale is
    /// responsible for elements size
    pub fn display_scale() -> f32 {
        1.0
    }

    #[cfg(macos)]
    pub fn display_scale() -> f32 {
        *Self::get().display_scale.lock().unwrap()
    }

    #[cfg(ios)]
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
