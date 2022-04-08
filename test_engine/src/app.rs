use std::{
    ffi::{c_float, c_int},
    marker::PhantomData,
    os::raw::c_ulong,
    path::{Path, PathBuf},
};

use gl_wrapper::monitor::Monitor;
use gm::volume::GyroData;
use rtools::Unwrap;
use tokio::runtime::Runtime;
use ui::{input::touch::TouchEvent, Touch};

use crate::{game_view::GameView, Screen};

pub struct App<T> {
    pub screen: Unwrap<Screen>,
    runtime:    Runtime,
    _view:      PhantomData<T>,
}

impl<T: GameView + 'static> App<T> {
    fn create_screen(&mut self, assets_path: &Path, monitor: Monitor) {
        self.runtime.block_on(async {
            let mut screen = Screen::new(assets_path, monitor.resolution);

            screen.ui.set_view(T::boxed());
            screen.ui.add_debug_view();

            screen.add_monitor(monitor);

            self.screen = screen.into();
        });
    }

    pub fn set_screen_size(&mut self, width: c_int, height: c_int) {
        self.runtime.block_on(async {
            self.screen.set_size((width, height).into());
        });
    }

    pub fn update_screen(&mut self) {
        self.runtime.block_on(async { self.screen.update() });
    }

    pub fn on_touch(&mut self, id: c_ulong, x: c_float, y: c_float, event: c_int) {
        #[allow(clippy::useless_conversion)]
        self.runtime.block_on(async {
            self.screen.ui.on_touch(Touch {
                id:       id.into(),
                position: (x, y).into(),
                event:    TouchEvent::from_int(event),
            })
        });
    }

    pub fn set_gyro(&mut self, pitch: c_float, roll: c_float, yaw: c_float) {
        self.runtime.block_on(async {
            self.screen.on_gyro_changed(GyroData { pitch, roll, yaw });
        });
    }

    #[allow(clippy::too_many_arguments)]
    pub fn set_monitor(
        &mut self,
        ppi: c_int,
        scale: c_float,
        refresh_rate: c_int,
        resolution_x: c_int,
        resolution_y: c_int,
        width: c_float,
        height: c_float,
        diagonal: c_float,
    ) {
        let monitor = Monitor::new(
            "Phone screen".into(),
            ppi as _,
            scale,
            refresh_rate as _,
            (resolution_x, resolution_y).into(),
            (width, height).into(),
            diagonal as _,
        );

        error!("{:?}", &monitor);
        dbg!(&monitor);

        self.create_screen(&PathBuf::new(), monitor);
    }
}

impl<T: GameView> Default for App<T> {
    fn default() -> Self {
        Self {
            screen:  Default::default(),
            runtime: tokio::runtime::Runtime::new().unwrap(),
            _view:   Default::default(),
        }
    }
}
