#![allow(clippy::mismatched_target_os)]

use core::ffi::{c_float, c_int};
use std::path::{Path, PathBuf};

use gl_wrapper::monitor::Monitor;
use gm::volume::GyroData;
use rtools::{init_log, platform::Platform, Unwrap};
use tokio::{
    runtime::Runtime,
    sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
};
use ui::{
    input::{ControlButton, KeyEvent, KeyState, KeyboardButton, TouchEvent, UIEvents},
    refs::Own,
    Touch, View,
};

use crate::Screen;

#[repr(C)]
pub enum TestEngineAction {
    None = 0,
    OpenKeyboard = 1,
    CloseKeyboard = 2,
}

#[repr(C)]
pub enum MobileKeyEvent {
    Letter = 0,
    Backspace = 1,
}

pub struct App {
    pub screen:      Unwrap<Own<Screen>>,
    runtime:         Runtime,
    _touch_sender:   UnboundedSender<Touch>,
    _touch_receiver: UnboundedReceiver<Touch>,
    _gyro_sender:    UnboundedSender<GyroData>,
    _gyro_receiver:  UnboundedReceiver<GyroData>,
}

impl App {
    fn create_screen(&mut self, assets_path: &Path, monitor: Monitor, view: Own<dyn View>) {
        self.runtime.block_on(async {
            let mut screen = Screen::new(monitor.resolution, assets_path, view);

            screen.add_monitor(monitor);

            self.screen = Unwrap::from(screen);
        });
    }

    pub fn set_screen_size(&mut self, width: c_int, height: c_int) {
        self.runtime.block_on(async {
            self.screen.set_size((width, height).into());
        });
    }

    pub fn update_screen(&mut self) -> TestEngineAction {
        self.runtime.block_on(async {
            if Platform::ANDROID {
                while let Ok(touch) = self._touch_receiver.try_recv() {
                    self.screen.ui.on_touch(touch);
                }
                while let Ok(gyro) = self._gyro_receiver.try_recv() {
                    self.screen.on_gyro_changed(gyro);
                }
            }
            self.screen.update()
        })
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

    pub fn add_key(&mut self, ch: u8, event: MobileKeyEvent) {
        self.runtime.block_on(async {
            let button = match event {
                MobileKeyEvent::Letter => KeyboardButton::Letter(ch as char),
                MobileKeyEvent::Backspace => ControlButton::Backspace.into(),
            };
            let event = KeyEvent {
                button,
                state: KeyState::Press,
            };
            UIEvents::get().key_pressed.trigger(event);
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
        view: Own<dyn View>,
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

        trace!("{:?}", &monitor);

        self.create_screen(&PathBuf::new(), monitor, view);
    }
}

impl Default for App {
    fn default() -> Self {
        init_log(false, 4);

        let (_touch_sender, _touch_receiver) = unbounded_channel::<Touch>();
        let (_gyro_sender, _gyro_receiver) = unbounded_channel::<GyroData>();
        Self {
            screen: Default::default(),
            runtime: tokio::runtime::Runtime::new().unwrap(),
            _touch_sender,
            _touch_receiver,
            _gyro_sender,
            _gyro_receiver,
        }
    }
}
