use std::path::PathBuf;

use rtools::init_log;
use test_engine::{app_core::AppCore, gm::flat::Size, paths::home, App};
use ui::{
    refs::{enable_ref_stats_counter, Own},
    View,
};

use crate::benchmark::UIDebugView;

pub struct TestApp {
    core: AppCore,
}

impl App for TestApp {
    fn setup() {
        enable_ref_stats_counter(true);
        init_log(false, 4);
    }

    fn screen_size() -> Size {
        (1000, 600).into()
    }

    fn assets_path() -> PathBuf {
        home().join("test_engine")
    }

    fn make_root_view() -> Own<dyn View> {
        Own::<UIDebugView>::default()
    }

    fn core(&mut self) -> &mut AppCore {
        &mut self.core
    }
}

#[cfg(desktop)]
impl Default for TestApp {
    fn default() -> Self {
        Self {
            core: Self::make_core(),
        }
    }
}

#[cfg(mobile)]
pub mod mobile {

    use std::ffi::{c_float, c_int};

    use test_engine::{app_core::AppCore, App};

    use crate::test_game::TestApp;

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

            let core = AppCore::new(
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
            Box::new(Self { core })
        }
    }
}
