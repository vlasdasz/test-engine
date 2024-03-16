use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use anyhow::Result;
use dispatch::on_main;
use gm::{
    flat::{Point, Size},
    LossyConvert, Platform,
};
use log::error;
use refs::{MainLock, Rglica};
use tokio::sync::oneshot::Receiver;
use wgpu::{BindGroupLayout, Device, Queue};
use winit::{
    dpi::PhysicalSize,
    event::{Event, MouseScrollDelta, WindowEvent},
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::WindowBuilder,
};

use crate::{
    app::App,
    render::state::{State, DEVICE, QUEUE},
    Screenshot,
};

static APP: MainLock<Option<WGPUApp>> = MainLock::const_new();

pub struct WGPUApp {
    pub state:  State,
    event_loop: Option<EventLoop<()>>,
    close:      AtomicBool,
}

impl WGPUApp {
    pub(crate) fn current() -> &'static mut Self {
        APP.get_mut().as_mut().expect("App has not been initialized yet.")
    }

    pub fn device() -> &'static Device {
        DEVICE.get_mut().as_mut().expect("Device has not been initialized yet.")
    }

    pub fn queue() -> &'static Queue {
        QUEUE.get_mut().as_mut().expect("Queue has not been initialized yet.")
    }

    pub fn screen_scale() -> f64 {
        Self::current().state.window.scale_factor()
    }

    pub fn close() {
        on_main(|| {
            Self::current().close.store(true, Ordering::Relaxed);
        });
    }

    pub async fn start(app: Box<dyn App>) -> Result<()> {
        let event_loop = EventLoop::new()?;

        let window = Arc::new(WindowBuilder::new().with_title("Test Engine").build(&event_loop)?);

        let scale: u32 = window.scale_factor().lossy_convert();

        _ = window.request_inner_size(PhysicalSize::new(800 * scale, 600 * scale));

        let state = State::new(app, window.clone()).await?;

        assert!(APP.is_none(), "Another instance of App already exists.");

        *APP.get_mut() = Self {
            state,
            event_loop: event_loop.into(),
            close: Default::default(),
        }
        .into();

        let app = Self::current();

        app.state.app.set_wgpu_app(Rglica::from_ref(app));
        app.state.app.window_ready();
        app.start_event_loop()
    }

    fn start_event_loop(&mut self) -> Result<()> {
        self.event_loop.take().unwrap().run(|event, elwt| match event {
            Event::WindowEvent { event, window_id } if window_id == self.state.window.id() => match event {
                WindowEvent::CloseRequested => elwt.exit(),
                WindowEvent::CursorMoved { position, .. } => {
                    self.state.app.mouse_moved((position.x, position.y).into());
                }
                WindowEvent::MouseInput { state, button, .. } => {
                    self.state.app.mouse_event(state, button);
                }
                WindowEvent::Touch(touch) => {
                    self.state.app.touch_event(touch);
                }
                WindowEvent::MouseWheel { delta, .. } => match delta {
                    MouseScrollDelta::LineDelta(x, y) => {
                        let point: Point = (x, y).into();
                        self.state.app.mouse_scroll(point * 28);
                    }
                    MouseScrollDelta::PixelDelta(delta) => {
                        self.state.app.mouse_scroll((delta.x, delta.y).into())
                    }
                },
                WindowEvent::KeyboardInput { event, .. } => {
                    if event.physical_key == PhysicalKey::Code(KeyCode::Escape) {
                        elwt.exit()
                    }
                    self.state.app.key_event(event);
                }
                WindowEvent::DroppedFile(path) => {
                    self.state.app.dropped_file(path);
                }
                WindowEvent::Resized(physical_size) => {
                    self.state.resize(physical_size);
                }
                WindowEvent::ScaleFactorChanged {
                    scale_factor,
                    inner_size_writer,
                } => {
                    dbg!(&scale_factor);
                    dbg!(&inner_size_writer);
                }
                WindowEvent::RedrawRequested => {
                    if self.close.load(Ordering::Relaxed) {
                        elwt.exit();
                    }

                    self.state.update();

                    match self.state.render() {
                        Ok(()) => {}
                        Err(e) => error!("Render error: {e:?}"),
                    };
                }
                _ => {}
            },
            Event::AboutToWait => {
                self.state.window.request_redraw();
            }
            _ => {}
        })?;

        Ok(())
    }

    pub fn set_title(&self, title: impl ToString) {
        if Platform::DESKTOP {
            self.state.window.set_title(&title.to_string());
        }
    }

    pub fn set_window_size(&self, size: impl Into<Size<u32>>) {
        let size = size.into();
        let _ = self.state.window.request_inner_size(PhysicalSize::new(size.width, size.height));
    }

    pub fn request_read_display(&self) -> Receiver<Screenshot> {
        self.state.request_read_display()
    }

    pub fn path_layout() -> &'static BindGroupLayout {
        &Self::current().state.drawer.path_state.color_size_layout
    }

    pub fn fps(&self) -> f32 {
        self.state.fps
    }
}
