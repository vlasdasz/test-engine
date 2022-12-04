use std::{ffi::c_int, ops::Deref, os::raw::c_float};

use rtools::init_log;
use test_engine::{app::App, paths::home};
use ui::refs::{enable_ref_stats_counter, Own};

use crate::benchmark::UIDebugView;

pub struct TestApp {
    pub app: App,
}

impl TestApp {
    fn setup() {
        enable_ref_stats_counter(true);
        init_log(false, 4);
    }

    #[cfg(desktop)]
    pub fn launch(&mut self) {
        self.app.screen.start_main_loop();
    }
}

#[cfg(mobile)]
impl TestApp {
    pub fn new(
        ppi: c_int,
        scale: c_float,
        refresh_rate: c_int,
        resolution_x: c_int,
        resolution_y: c_int,
        width: c_float,
        height: c_float,
        diagonal: c_float,
    ) -> Box<Self> {
        Self::setup();

        let app = App::new(
            ppi,
            scale,
            refresh_rate,
            resolution_x,
            resolution_y,
            width,
            height,
            diagonal,
            Self::make_root_view(),
        );
        Box::new(Self { app })
    }
}

#[cfg(desktop)]
impl Default for TestApp {
    fn default() -> Self {
        Self::setup();
        let app = App::new((1000, 600), home().join("test_engine"), Self::make_root_view());
        Self { app }
    }
}

impl TestApp {
    fn make_root_view() -> Own<UIDebugView> {
        Default::default()
    }
}
