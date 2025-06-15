use std::{
    ops::{Deref, DerefMut},
    path::PathBuf,
    sync::{
        Mutex, OnceLock,
        atomic::{AtomicBool, AtomicU32, Ordering},
    },
};

use gm::{
    Platform, ToF32,
    color::Color,
    flat::{Point, Rect, Size},
};
use refs::{Own, Weak, assert_main_thread};
use window::Window;

use crate::{DEBUG_VIEW, Keymap, RootView, TouchStack, UIEvent, View, ViewData, WeakView};

static UI_MANAGER: OnceLock<UIManager> = OnceLock::new();

#[cfg(ios)]
static IOS_KEYBOARD_INIT: std::sync::Once = std::sync::Once::new();

pub struct UIManager {
    pub(crate) root_view:     Own<RootView>,
    pub(crate) deleted_views: Mutex<Vec<Own<dyn View>>>,

    touch_disabled: AtomicBool,

    draw_debug_frames: AtomicBool,

    scale:         AtomicU32,
    scale_changed: UIEvent<f32>,

    on_scroll:    UIEvent<Point>,
    on_drop_file: UIEvent<PathBuf>,

    draw_touches: AtomicBool,

    keymap: Own<Keymap>,

    selected_view: Mutex<WeakView>,
}

impl UIManager {
    pub const ROOT_VIEW_Z_OFFSET: f32 = 0.5;
    pub const MODAL_Z_OFFSET: f32 = 0.4;
    pub const DEBUG_Z_OFFSET: f32 = 0.3;

    pub const fn subview_z_offset() -> f32 {
        0.000_01
    }

    pub const fn additional_z_offset() -> f32 {
        Self::subview_z_offset() / 100.0
    }

    pub fn fps() -> f32 {
        Window::current().fps()
    }

    pub fn frame_drawn() -> u32 {
        Window::current().frame_drawn()
    }

    pub fn scale() -> f32 {
        f32::from_le_bytes(Self::get().scale.load(Ordering::Relaxed).to_le_bytes())
    }

    pub fn set_scale(scale: impl ToF32) {
        assert_main_thread();
        let sf = Self::get();
        let scale = scale.to_f32();

        sf.scale.store(u32::from_le_bytes(scale.to_le_bytes()), Ordering::Relaxed);
        sf.scale_changed.trigger(scale);
    }

    pub fn on_scale_changed<U>(subscriber: Weak<U>, mut cb: impl FnMut(f32) + Send + 'static) {
        Self::get().scale_changed.val(subscriber, move |scale| {
            cb(scale);
        });
    }

    pub fn unselect_view() {
        let this = Self::get();
        let mut selected_view = this.selected_view.lock().unwrap();
        if selected_view.is_null() {
            return;
        }
        selected_view.base_view_mut().is_selected = false;
        selected_view.on_selection_changed(false);
        *selected_view = Weak::default();
    }

    pub fn set_selected(mut view: WeakView, selected: bool) {
        let this = Self::get();

        let mut selected_view = this.selected_view.lock().unwrap();

        if let Some(selected) = selected_view.get_mut() {
            selected.on_selection_changed(false);
            *selected_view = Weak::default();
        }

        if selected {
            *selected_view = view;
        }

        view.base_view_mut().is_selected = selected;
        view.on_selection_changed(selected);
    }
}

impl UIManager {
    fn init() -> Self {
        let mut root_view = Own::<RootView>::default();
        root_view.base_view_mut().view_label = "Root view".to_string();
        root_view.setup_root();

        Self {
            root_view,
            deleted_views: Mutex::default(),
            touch_disabled: false.into(),
            draw_debug_frames: false.into(),
            scale: AtomicU32::new(u32::from_le_bytes(1.0f32.to_le_bytes())),
            scale_changed: UIEvent::default(),
            on_scroll: UIEvent::default(),
            on_drop_file: UIEvent::default(),
            draw_touches: false.into(),
            keymap: Own::default(),
            selected_view: Mutex::new(Weak::default()),
        }
    }

    pub(crate) fn get() -> &'static Self {
        UI_MANAGER.get_or_init(Self::init)
    }

    pub fn window_resolution() -> Size {
        let size = if Platform::IOS {
            Window::render_size()
        } else {
            Window::inner_size()
        };
        (size.width, size.height).into()
    }

    pub fn display_scale() -> f32 {
        Window::screen_scale()
    }

    pub fn debug_view() -> Option<&'static mut dyn View> {
        DEBUG_VIEW.get_mut().as_mut().map(DerefMut::deref_mut)
    }

    pub fn root_view() -> Weak<RootView> {
        Self::get().root_view.weak()
    }

    pub fn root_view_static() -> &'static RootView {
        Self::get().root_view.deref()
    }

    pub fn free_deleted_views() {
        Self::get().deleted_views.lock().unwrap().clear();
        TouchStack::get().clear_freed();
    }

    pub fn enable_debug_frames() {
        Self::get().draw_debug_frames.store(true, Ordering::Relaxed);
    }

    pub fn disable_debug_frames() {
        Self::get().draw_debug_frames.store(false, Ordering::Relaxed);
    }

    pub fn should_draw_debug_frames() -> bool {
        Self::get().draw_debug_frames.load(Ordering::Relaxed)
    }

    pub fn draw_touches() -> bool {
        Self::get().draw_touches.load(Ordering::Relaxed)
    }

    pub fn set_display_touches(display: bool) {
        Self::get().draw_touches.store(display, Ordering::Relaxed);
    }

    pub fn keymap() -> &'static Keymap {
        Self::get().keymap.deref()
    }
}

impl UIManager {
    pub fn touch_disabled() -> bool {
        Self::get().touch_disabled.load(Ordering::Relaxed)
    }

    fn disable_touch() {
        Self::get().touch_disabled.store(true, Ordering::Relaxed);
    }

    fn enable_touch() {
        Self::get().touch_disabled.store(false, Ordering::Relaxed);
    }
}

impl UIManager {
    /// There are 2 types of scale
    /// Display scale - constant for display on mac and iPhones, always 1 on
    /// other OS (probably) UI scale - adjustable in runtime
    pub fn rescale_frame(rect: &Rect) -> Rect {
        let scale = Self::display_scale();

        let rect: Rect = (
            rect.origin.x * scale,
            (Self::window_resolution().height - rect.origin.y - rect.size.height) * scale,
            rect.size.width * scale,
            rect.size.height * scale,
        )
            .into();

        rect
    }

    pub fn open_keyboard(#[allow(unused_variables)] frame: &Rect) {
        #[cfg(ios)]
        {
            crate::ui_manager::IOS_KEYBOARD_INIT.call_once(|| {
                unsafe { crate::mobile::ios::test_engine_ios_init_text_field() };
            });

            unsafe {
                crate::mobile::ios::test_engine_ios_open_keyboard(
                    frame.origin.x,
                    frame.origin.y,
                    frame.size.width,
                    frame.size.height,
                )
            }
        }
    }

    pub fn close_keyboard() -> Option<String> {
        #[cfg(ios)]
        unsafe {
            let str_ptr = crate::mobile::ios::test_engine_ios_close_keyboard();
            let cstr = std::ffi::CStr::from_ptr(str_ptr);
            return cstr.to_string_lossy().into_owned().into();
        }

        #[cfg(not(ios))]
        None
    }

    pub fn set_view<T: View + 'static>(view: Own<T>) -> Weak<T> {
        let weak = view.weak();
        let mut root = UIManager::root_view();
        root.clear_root();
        let view = root.add_subview_to_root(view);
        if view.place().is_empty() {
            view.place().back();
        }
        weak
    }
}

impl UIManager {
    pub fn trigger_scroll(scroll: Point) {
        Self::get().on_scroll.trigger(scroll);
    }

    pub fn on_scroll<T: ?Sized>(subscriber: Weak<T>, action: impl FnMut(Point) + Send + 'static) {
        Self::get().on_scroll.val(subscriber, action);
    }

    pub fn trigger_drop_file(file: PathBuf) {
        Self::get().on_drop_file.trigger(file);
    }

    pub fn on_drop_file<T: ?Sized>(subscriber: Weak<T>, action: impl FnMut(PathBuf) + Send + 'static) {
        Self::get().on_drop_file.val(subscriber, action);
    }

    pub fn set_clear_color(color: impl Into<Color>) {
        Window::set_clear_color(color);
    }
}

pub struct TouchLock;

impl TouchLock {
    pub(crate) fn new() -> Self {
        UIManager::disable_touch();
        TouchLock
    }
}

impl Drop for TouchLock {
    fn drop(&mut self) {
        UIManager::enable_touch();
    }
}
