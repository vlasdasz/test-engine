use core::ffi::{c_float, c_int};
use std::path::{Path, PathBuf};

#[cfg(mobile)]
use gl_wrapper::monitor::Monitor;
#[cfg(desktop)]
use gl_wrapper::GLFWManager;
use gm::flat::Size;
#[cfg(mobile)]
use gm::volume::GyroData;
#[cfg(mobile)]
use rtools::{init_log, platform::Platform, Unwrap};
#[cfg(mobile)]
use tokio::{
    runtime::Runtime,
    sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
};
#[cfg(mobile)]
use ui::input::{ControlButton, KeyEvent, KeyState, KeyboardButton, TouchEvent, UIEvents};
#[cfg(mobile)]
use ui::refs::set_current_thread_as_main;
use ui::{refs::Own, Touch, View};

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
    pub screen:      Own<Screen>,
    #[cfg(mobile)]
    runtime:         Runtime,
    #[cfg(mobile)]
    _touch_sender:   UnboundedSender<Touch>,
    #[cfg(mobile)]
    _touch_receiver: UnboundedReceiver<Touch>,
    #[cfg(mobile)]
    _gyro_sender:    UnboundedSender<GyroData>,
    #[cfg(mobile)]
    _gyro_receiver:  UnboundedReceiver<GyroData>,
}

impl App {
    #[cfg(mobile)]
    pub fn set_screen_size(&mut self, width: c_int, height: c_int) {
        self.runtime.block_on(async {
            self.screen.set_size((width, height).into());
        });
    }

    #[cfg(mobile)]
    pub fn update_screen(&mut self) -> TestEngineAction {
        self.runtime.block_on(async {
            #[cfg(android)]
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

    #[cfg(mobile)]
    pub fn on_touch(&mut self, id: u64, x: c_float, y: c_float, event: c_int) {
        let touch = Touch {
            id,
            position: (x, y).into(),
            event: TouchEvent::from_int(event),
        };

        if Platform::ANDROID {
            #[cfg(android)]
            if let Err(err) = self._touch_sender.send(touch) {
                error!("Error sending touch: {:?}", err);
            }
        } else {
            self.runtime.block_on(async {
                self.screen.ui.on_touch(touch);
            });
        };
    }

    #[cfg(mobile)]
    pub fn set_gyro(&mut self, pitch: c_float, roll: c_float, yaw: c_float) {
        let gyro = GyroData { pitch, roll, yaw };

        if Platform::ANDROID {
            #[cfg(android)]
            if let Err(err) = self._gyro_sender.send(gyro) {
                error!("Error sending gyro: {:?}", err);
            }
        } else {
            self.runtime.block_on(async {
                self.screen.on_gyro_changed(gyro);
            });
        }
    }

    #[cfg(mobile)]

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
}

impl App {
    #[cfg(desktop)]
    pub fn new(size: impl Into<Size>, assets_path: impl Into<PathBuf>, root_view: Own<dyn View>) -> Self {
        let glfw = GLFWManager::default();
        trace!("GLFWManager: OK");

        let monitor = glfw.monitors.first().unwrap().clone();
        let screen = Screen::new(monitor, assets_path, root_view, glfw, size);
        Self { screen }
    }

    #[cfg(mobile)]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ppi: c_int,
        scale: c_float,
        refresh_rate: c_int,
        resolution_x: c_int,
        resolution_y: c_int,
        width: c_float,
        height: c_float,
        diagonal: c_float,
        view: Own<dyn View>,
    ) -> Self {
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

        let (_touch_sender, _touch_receiver) = unbounded_channel::<Touch>();
        let (_gyro_sender, _gyro_receiver) = unbounded_channel::<GyroData>();

        let runtime = tokio::runtime::Runtime::new().unwrap();

        let mut screen: Option<Own<Screen>> = None;

        let screen = runtime.block_on(async {
            set_current_thread_as_main();
            Screen::new(monitor, PathBuf::new(), view).into()
        });

        //let screen = screen.unwrap();

        Self {
            screen,
            runtime,
            _touch_sender,
            _touch_receiver,
            _gyro_sender,
            _gyro_receiver,
        }
    }
}
