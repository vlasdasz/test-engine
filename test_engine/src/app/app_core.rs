use ui::refs::Own;

#[cfg(mobile)]
use crate::app::mobile::MobileStuff;
use crate::Screen;

#[repr(C)]
pub enum TestEngineAction {
    None = 0,
    OpenKeyboard = 1,
    CloseKeyboard = 2,
}

pub struct AppCore {
    pub screen: Own<Screen>,
    #[cfg(mobile)]
    pub mobile: MobileStuff,
}

#[cfg(desktop)]
mod desktop {
    use std::path::PathBuf;

    use gl_wrapper::GLFWManager;
    use gm::flat::Size;
    use ui::{refs::Own, View};

    use crate::{AppCore, Screen};

    impl AppCore {
        pub fn new(size: impl Into<Size>, assets_path: impl Into<PathBuf>, root_view: Own<dyn View>) -> Self {
            trace!("AppCore::new");
            let size = size.into();

            let glfw = GLFWManager::new(size);
            trace!("GLFWManager: OK");

            let monitor = glfw.monitors.first().unwrap().clone();
            trace!("Monitor: OK");
            let screen = Screen::new(monitor, assets_path, root_view, glfw, size);
            trace!("Screen: OK");
            Self { screen }
        }
    }
}

#[cfg(mobile)]
pub mod mobile {
    use std::{
        ffi::{c_float, c_int},
        path::PathBuf,
    };

    use gl_wrapper::monitor::Monitor;
    use gm::volume::GyroData;
    use rtools::platform::Platform;
    use tokio::{
        runtime::Runtime,
        sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
    };
    use ui::{
        input::{ControlButton, KeyEvent, KeyState, KeyboardButton, UIEvents},
        refs::{set_current_thread_as_main, Own},
        Touch, View,
    };

    use crate::{
        app::{AppCore, TestEngineAction},
        Screen,
    };

    #[repr(C)]
    pub enum MobileKeyEvent {
        Letter = 0,
        Backspace = 1,
    }

    pub struct MobileStuff {
        runtime:         Runtime,
        _touch_sender:   UnboundedSender<Touch>,
        _touch_receiver: UnboundedReceiver<Touch>,
        _gyro_sender:    UnboundedSender<GyroData>,
        _gyro_receiver:  UnboundedReceiver<GyroData>,
    }

    impl AppCore {
        pub fn set_screen_size(&mut self, width: c_int, height: c_int) {
            self.mobile.runtime.block_on(async {
                self.screen.set_size((width, height));
            });
        }

        pub fn update_screen(&mut self) -> TestEngineAction {
            self.mobile.runtime.block_on(async {
                if Platform::ANDROID {
                    while let Ok(touch) = self.mobile._touch_receiver.try_recv() {
                        self.screen.ui.on_touch(touch);
                    }
                    while let Ok(gyro) = self.mobile._gyro_receiver.try_recv() {
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
                event: event.into(),
            };

            if Platform::ANDROID {
                if let Err(err) = self.mobile._touch_sender.send(touch) {
                    error!("Error sending touch: {:?}", err);
                }
            } else {
                self.mobile.runtime.block_on(async {
                    self.screen.ui.on_touch(touch);
                });
            };
        }

        pub fn set_gyro(&mut self, pitch: c_float, roll: c_float, yaw: c_float) {
            let gyro = GyroData { pitch, roll, yaw };

            if Platform::ANDROID {
                if let Err(err) = self.mobile._gyro_sender.send(gyro) {
                    error!("Error sending gyro: {:?}", err);
                }
            } else {
                self.mobile.runtime.block_on(async {
                    self.screen.on_gyro_changed(gyro);
                });
            }
        }

        pub fn add_key(&mut self, ch: u8, event: MobileKeyEvent) {
            self.mobile.runtime.block_on(async {
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

            let screen = runtime.block_on(async {
                set_current_thread_as_main();
                Screen::new(monitor, PathBuf::new(), view).into()
            });

            let mobile = MobileStuff {
                runtime,
                _touch_sender,
                _touch_receiver,
                _gyro_sender,
                _gyro_receiver,
            };

            Self { screen, mobile }
        }
    }
}
