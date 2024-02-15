use core::ptr::from_mut;
use std::{ops::DerefMut, path::PathBuf, ptr::null_mut, sync::atomic::Ordering};

use chrono::Utc;
use dispatch::from_main;
use gm::{
    flat::{Point, Rect, Size},
    Color,
};
use refs::{assert_main_thread, Own};
use rest::API;
use ui::{
    UIManager, View, ViewData, ViewFrame, ViewSetup, ViewSubviews, ViewTest, MICROSECONDS_IN_ONE_SECOND,
};
use ui_views::{DebugView, SHOW_DEBUG_VIEW};

use crate::{app::TestEngineAction, assets::Assets, ui_layer::UILayer};

static mut SCREEN: *mut Screen = null_mut();

pub struct Screen {
    pub(crate) ui: Own<UILayer>,
}

impl Screen {
    #[cfg(desktop)]
    fn setup_events(&mut self) {
        self.ui.setup_events();

        // SystemEvents::get().size_changed.val(move |size| {
        //     this.size_changed(size);
        // });
        //
        // SystemEvents::get().frame_drawn.sub(move || {
        //     this.update();
        // });
    }

    fn init(&mut self, #[cfg(desktop)] window_size: Size<u32>, view: Own<dyn View>) {
        UIManager::root_view().add_subview(view).place().back();

        if SHOW_DEBUG_VIEW.load(Ordering::Relaxed) {
            let mut debug_view = DebugView::new();
            debug_view.priority = 10;
            let weak = debug_view.weak();
            UIManager::root_view().add_subview(debug_view);
            Screen::current().ui.debug_view = weak;
        }
        #[cfg(desktop)]
        {
            self.size_changed(window_size);
        }
    }
}

impl Screen {
    pub fn current() -> &'static mut Screen {
        assert_main_thread();
        unsafe {
            assert!(!SCREEN.is_null(), "Screen was not initialized");
            SCREEN.as_mut().unwrap()
        }
    }

    pub fn read_pixel(_pos: Point) -> Color {
        // GLWrapper::read_pixel(
        //     UIManager::rescale_frame(&Rect {
        //         origin: pos,
        //         ..Default::default()
        //     })
        //     .origin,
        // )
        todo!()
    }

    #[cfg(desktop)]
    pub fn take_screenshot() {
        //Self::current().glfw.take_screenshot()
    }

    #[cfg(mobile)]
    pub fn take_screenshot() {
        todo!("Take screenshot is not implemented for mobile yet")
    }

    #[cfg(desktop)]
    pub fn set_title(_title: impl ui::ToLabel + Send + Sync + 'static) {
        //dispatch::on_main(move ||
        // Screen::current().glfw.set_window_title(&title.to_label()));
    }

    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    fn calculate_fps(&mut self) {
        let now = Utc::now().timestamp_micros();

        let interval = now - self.ui.prev_time;
        self.ui.prev_time = now;

        self.ui.frame_time = interval as f32 / MICROSECONDS_IN_ONE_SECOND as f32;
        self.ui.fps = (1.0 / self.ui.frame_time) as u64;

        // #[cfg(desktop)]
        // {
        //     let size = self.glfw.get_size();
        //     Self::set_title(format!(
        //         "{:<8} x   {:<8}    {:<8} FPS",
        //         size.width, size.height, self.ui.fps
        //     ));
        // }

        if SHOW_DEBUG_VIEW.load(Ordering::Relaxed) && self.ui.debug_view.is_ok() {
            let fps = self.ui.fps;
            self.ui.debug_view.fps.trigger(fps);
            if API::is_ok() {
                self.ui.debug_view.set_custom("URL:", API::base_url());
            } else {
                self.ui.debug_view.set_custom("URL:", "API not initizlized");
            }
        }
    }

    pub fn update(&mut self) -> TestEngineAction {
        self.calculate_fps();

        let mut root_view = UIManager::root_view();
        let root_frame: Rect = UIManager::root_view_size().into();
        root_view.set_frame(root_frame);

        dispatch::invoke_dispatched();

        UIManager::update();

        // TODO: tis ugly
        if UIManager::get().close_keyboard.load(Ordering::Relaxed) {
            UIManager::get().close_keyboard.store(false, Ordering::Relaxed);
            TestEngineAction::CloseKeyboard
        } else if UIManager::get().open_keyboard.load(Ordering::Relaxed) {
            UIManager::get().open_keyboard.store(false, Ordering::Relaxed);
            TestEngineAction::OpenKeyboard
        } else {
            TestEngineAction::None
        }
    }

    #[cfg(desktop)]
    pub fn set_size(&mut self, _size: impl Into<Size<u32>>) {}

    pub fn size_changed(&mut self, size: Size<u32>) {
        trace!("Size changed: {:?}", size);
        UIManager::set_window_size(size);

        self.update();
    }

    #[cfg(mobile)]
    pub(crate) fn on_gyro_changed(&mut self, gyro: gm::volume::GyroData) {
        // error!("GyroData: {:?}", gyro);

        ui::input::UIEvents::get().gyro_changed.trigger(gyro);

        let Some(level) = &mut self.ui.level else {
            return;
        };
        level.on_gyro_changed(gyro);
    }

    #[cfg(desktop)]
    pub fn start_main_loop(&mut self, _callback: impl FnOnce()) -> anyhow::Result<()> {
        self.setup_events();
        Ok(())
    }

    #[allow(unused_variables)]
    pub async fn set_test_view<T: View + ViewTest + Default + 'static>(width: u32, height: u32) {
        from_main(move || {
            let view = T::new();
            let mut root = UIManager::root_view();
            root.remove_all_subviews();
            let view = root.add_subview(view);
            view.place().back();
            #[cfg(desktop)]
            Screen::current().set_size((width, height));
        })
        .await
    }
}

impl Screen {
    pub fn new(
        assets_path: impl Into<PathBuf>,
        root_view: Own<dyn View>,
        #[cfg(desktop)] window_size: Size<u32>,
    ) -> Own<Self> {
        trace!("Creating screen");

        Assets::init(assets_path);
        trace!("Assets: Ok");

        let ui = Own::<UILayer>::default();
        trace!("UILayer: OK");

        let mut screen = Own::new(Self { ui });

        unsafe {
            SCREEN = from_mut::<Screen>(screen.deref_mut());
        }

        screen.init(
            #[cfg(desktop)]
            window_size,
            root_view,
        );

        screen
    }
}
