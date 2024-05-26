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
    state::{State, DRAWER, SURFACE},
    surface::Surface,
    Screenshot, WGPUDrawer,
};

static APP: MainLock<Option<WGPUApp>> = MainLock::new();

#[cfg(target_os = "android")]
pub type Events = winit::platform::android::activity::AndroidApp;

#[cfg(not(target_os = "android"))]
pub type Events = ();

pub struct WGPUApp {
    pub state:  State,
    event_loop: Option<EventLoop<Events>>,
    close:      AtomicBool,
}

impl WGPUApp {
    pub fn current() -> &'static mut Self {
        APP.get_mut().as_mut().expect("App has not been initialized yet.")
    }

    pub(crate) fn surface() -> &'static Surface {
        SURFACE.get_mut().as_mut().expect("Sufrace has not been initialized yet.")
    }

    pub(crate) fn surface_mut() -> &'static mut Surface {
        SURFACE.get_mut().as_mut().expect("Sufrace has not been initialized yet.")
    }

    pub fn device() -> &'static Device {
        &Self::surface().device
    }

    pub fn queue() -> &'static Queue {
        &Self::surface().queue
    }

    pub fn drawer() -> &'static mut WGPUDrawer {
        DRAWER.get_mut().as_mut().expect("WGPUDrawer has not been initialized yet.")
    }

    pub fn screen_scale() -> f64 {
        Self::current().state.window.scale_factor()
    }

    pub fn close() {
        on_main(|| {
            Self::current().close.store(true, Ordering::Relaxed);
        });
    }

    async fn start_internal(app: Box<dyn App>, event_loop: EventLoop<Events>) -> Result<()> {
        dbg!("start_internal");

        let window = Arc::new(WindowBuilder::new().with_title("Test Engine").build(&event_loop)?);

        dbg!("start_internal");

        let scale: u32 = window.scale_factor().lossy_convert();

        dbg!("start_internal");

        _ = window.request_inner_size(PhysicalSize::new(1200 * scale, 1000 * scale));

        dbg!("start_internal");

        let state = State::new(app, window.clone()).await?;

        dbg!("start_internal");

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

    #[cfg(not(target_os = "android"))]
    pub async fn start(app: Box<dyn App>) -> Result<()> {
        Self::start_internal(app, EventLoop::new()?).await
    }

    #[cfg(target_os = "android")]
    pub async fn start(app: Box<dyn App>, event_loop: EventLoop<Events>) -> Result<()> {
        Self::start_internal(app, event_loop).await
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
            Event::Resumed => {
                dbg!("resumed");
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
        &Self::drawer().path_drawer.color_size_layout
    }

    pub fn fps(&self) -> f32 {
        self.state.fps
    }

    pub fn frame_time(&self) -> f32 {
        self.state.frame_time
    }
}
