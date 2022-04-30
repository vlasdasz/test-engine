#![allow(clippy::mismatched_target_os)]

use core::ffi::{c_float, c_int};
use std::{
    marker::PhantomData,
    path::{Path, PathBuf},
};

use gl_wrapper::monitor::Monitor;
use gm::volume::GyroData;
use rtools::{platform::Platform, Unwrap};
use tokio::{
    runtime::Runtime,
    sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
};
use ui::{input::touch::TouchEvent, Touch};

use crate::{main_view::MainView, Screen};

pub struct App<T> {
    pub screen:      Unwrap<Screen>,
    runtime:         Runtime,
    _view:           PhantomData<T>,
    _touch_sender:   UnboundedSender<Touch>,
    _touch_receiver: UnboundedReceiver<Touch>,
    _gyro_sender:    UnboundedSender<GyroData>,
    _gyro_receiver:  UnboundedReceiver<GyroData>,
}

impl<T: MainView + 'static> App<T> {
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
        self.runtime.block_on(async {
            if Platform::ANDROID {
                while let Ok(touch) = self._touch_receiver.try_recv() {
                    self.screen.ui.on_touch(touch);
                }
                while let Ok(gyro) = self._gyro_receiver.try_recv() {
                    self.screen.on_gyro_changed(gyro);
                }
            }
            self.screen.update();
        });
    }

    pub fn on_touch(&mut self, id: u64, x: c_float, y: c_float, event: c_int) {
        let touch = Touch {
            id,
            position: (x, y).into(),
            event: TouchEvent::from_int(event),
        };

        if Platform::ANDROID {
            if let Err(err) = self._touch_sender.send(touch) {
                error!("Error sending touch: {:?}", err);
            }
        } else {
            self.runtime.block_on(async {
                self.screen.ui.on_touch(touch);
            });
        };
    }

    pub fn set_gyro(&mut self, pitch: c_float, roll: c_float, yaw: c_float) {
        let gyro = GyroData { pitch, roll, yaw };

        if Platform::ANDROID {
            if let Err(err) = self._gyro_sender.send(gyro) {
                error!("Error sending gyro: {:?}", err);
            }
        } else {
            self.runtime.block_on(async {
                self.screen.on_gyro_changed(gyro);
            });
        }
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

impl<T: MainView> Default for App<T> {
    fn default() -> Self {
        let (_touch_sender, _touch_receiver) = unbounded_channel::<Touch>();
        let (_gyro_sender, _gyro_receiver) = unbounded_channel::<GyroData>();
        Self {
            screen: Default::default(),
            runtime: tokio::runtime::Runtime::new().unwrap(),
            _view: Default::default(),
            _touch_sender,
            _touch_receiver,
            _gyro_sender,
            _gyro_receiver,
        }
    }
}
