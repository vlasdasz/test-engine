use std::sync::atomic::{AtomicBool, Ordering};

use gm::flat::Point;
use log::{debug, error};
use refs::{Rglica, main_lock::MainLock};
use winit::{
    application::ApplicationHandler,
    event::{MouseScrollDelta, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy},
    keyboard::{KeyCode, PhysicalKey},
    window::WindowId,
};

use crate::{Window, WindowEvents};

static APP_HANDLER: MainLock<Option<AppHandler>> = MainLock::new();

#[allow(clippy::large_enum_variant)]
enum AppHandlerState {
    Ready(Window),
    Init(Option<EventLoopProxy<Window>>),
}

impl AppHandlerState {
    fn window(&self) -> &Window {
        let Self::Ready(window) = self else {
            panic!("Window not init")
        };
        window
    }

    fn ready(&self) -> bool {
        !self.not_ready()
    }

    fn not_ready(&self) -> bool {
        matches!(self, Self::Init(_))
    }
}

pub struct AppHandler {
    state:                       AppHandlerState,
    pub(crate) te_window_events: Box<dyn WindowEvents>,
    pub(crate) close:            AtomicBool,
}

impl AppHandler {
    pub fn new(app: impl WindowEvents + 'static, event_loop: &EventLoop<Window>) -> &'static mut Self {
        let handler = APP_HANDLER.get_mut();

        *handler = Some(Self {
            state:            AppHandlerState::Init(Some(event_loop.create_proxy())),
            te_window_events: Box::new(app),
            close:            AtomicBool::new(false),
        });

        handler.as_mut().expect("Failed to get handler")
    }
}

impl AppHandler {
    pub(crate) fn close() {
        Self::current().close.store(true, Ordering::Relaxed);
    }

    pub(crate) fn current() -> &'static mut Self {
        APP_HANDLER
            .get_mut()
            .as_mut()
            .expect("AppHandler has not been initialized yet.")
    }

    pub(crate) fn window() -> &'static mut Window {
        let AppHandlerState::Ready(ref mut window) = Self::current().state else {
            panic!("Window is not initialized yet")
        };
        window
    }
}

impl ApplicationHandler<Window> for AppHandler {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if let AppHandlerState::Init(proxy) = &mut self.state
            && let Some(proxy) = proxy.take()
        {
            let mut win_attr = winit::window::Window::default_attributes();

            #[cfg(not(target_arch = "wasm32"))]
            {
                win_attr = win_attr.with_title("WebGPU example");
            }

            #[cfg(target_arch = "wasm32")]
            {
                use winit::platform::web::WindowAttributesExtWebSys;
                win_attr = win_attr.with_append(true);
            }

            let window = event_loop.create_window(win_attr).expect("create window err.");

            #[cfg(target_arch = "wasm32")]
            wasm_bindgen_futures::spawn_local(Window::start_internal(window, proxy));

            #[cfg(not(target_arch = "wasm32"))]
            pollster::block_on(Window::start_internal(window, proxy));
        }
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, window: Window) {
        self.state = AppHandlerState::Ready(window);
        self.te_window_events.set_window(Rglica::from_ref(self.state.window()));
        self.te_window_events.window_ready();

        AppHandler::current().te_window_events.resize(
            Window::inner_position(),
            Window::outer_position(),
            Window::inner_size(),
            Window::outer_size(),
        );
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::CursorMoved { position, .. } => {
                self.te_window_events.mouse_moved((position.x, position.y).into());
            }
            WindowEvent::MouseInput { state, button, .. } => {
                self.te_window_events.mouse_event(state, button);
            }
            WindowEvent::Touch(touch) => {
                self.te_window_events.touch_event(touch);
            }
            WindowEvent::MouseWheel { delta, .. } => match delta {
                MouseScrollDelta::LineDelta(x, y) => {
                    let point: Point = (x, y).into();
                    self.te_window_events.mouse_scroll(point * 28.0);
                }
                MouseScrollDelta::PixelDelta(delta) => {
                    self.te_window_events.mouse_scroll((delta.x, delta.y).into());
                }
            },
            WindowEvent::KeyboardInput { event, .. } => {
                if event.physical_key == PhysicalKey::Code(KeyCode::Escape) {
                    event_loop.exit();
                }
                self.te_window_events.key_event(event);
            }
            WindowEvent::DroppedFile(path) => {
                self.te_window_events.dropped_file(path);
            }
            WindowEvent::Resized(physical_size) => {
                if self.state.not_ready() {
                    return;
                }
                Self::window().state.resize(physical_size, event_loop);
            }
            WindowEvent::ScaleFactorChanged {
                scale_factor,
                inner_size_writer: _,
            } => {
                debug!("Scale factor: {scale_factor}");
            }
            WindowEvent::RedrawRequested => {
                if self.state.not_ready() {
                    return;
                }

                if self.close.load(Ordering::Relaxed) {
                    event_loop.exit();
                }

                Self::window().state.update();

                match Self::window().state.render() {
                    Ok(()) => {}
                    Err(e) => error!("Render error: {e:?}"),
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if self.state.ready() {
            Window::winit_window().request_redraw();
        }
    }
}
