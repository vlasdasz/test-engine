use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use anyhow::{anyhow, Result};
use dispatch::on_main;
use gm::{
    flat::{Point, Size},
    LossyConvert, Platform,
};
use log::{error, info};
use refs::{MainLock, Rglica};
use tokio::sync::oneshot::Receiver;
use wgpu::{
    Adapter, Backends, BindGroupLayout, CompositeAlphaMode, Device, DeviceDescriptor, Features, Instance,
    InstanceDescriptor, Limits, PresentMode, Queue, RequestAdapterOptions, SurfaceConfiguration,
    TextureUsages,
};
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::{MouseScrollDelta, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowAttributes, WindowId},
};

use crate::{
    app::App,
    state::{State, TEXTURE_FORMAT},
    surface::Surface,
    Screenshot, WGPUDrawer,
};

const ENABLE_VSYNC: bool = true;

static APP: MainLock<Option<WGPUApp>> = MainLock::new();

#[cfg(target_os = "android")]
pub type Events = winit::platform::android::activity::AndroidApp;

#[cfg(not(target_os = "android"))]
pub type Events = ();

pub struct WGPUApp {
    pub state: State,

    pub window_size: Size,

    pub(crate) config: SurfaceConfiguration,
    pub(crate) device: Device,
    pub(crate) queue:  Queue,

    adapter:  Adapter,
    instance: Instance,

    pub(crate) resumed: bool,

    pub(crate) surface: Option<Surface>,
    pub(crate) drawer:  Option<WGPUDrawer>,

    close: AtomicBool,
}

impl WGPUApp {
    pub fn current() -> &'static mut Self {
        APP.get_mut().as_mut().expect("App has not been initialized yet.")
    }

    pub fn device() -> &'static Device {
        &Self::current().device
    }

    pub fn queue() -> &'static Queue {
        &Self::current().queue
    }

    pub fn window() -> &'static Window {
        &Self::current().surface.as_ref().expect("No surface yet").window
    }

    pub fn drawer() -> &'static mut WGPUDrawer {
        Self::current().drawer.as_mut().expect("Drawer has not been initialized yet.")
    }

    pub fn screen_scale() -> f64 {
        Self::window().scale_factor()
    }

    pub fn close() {
        on_main(|| {
            Self::current().close.store(true, Ordering::Relaxed);
        });
    }

    pub(crate) fn create_surface_and_window(&mut self, event_loop: &ActiveEventLoop) -> Result<bool> {
        let window =
            Arc::new(event_loop.create_window(WindowAttributes::default().with_title("Test Engine"))?);

        let scale = window.scale_factor();

        _ = window.request_inner_size(PhysicalSize::new(1200.0 * scale, 1000.0 * scale));

        self.config.width = (1200.0 * scale).lossy_convert();
        self.config.height = (1000.0 * scale).lossy_convert();

        let Some(surface) = Surface::new(&self.instance, &self.adapter, &self.device, &self.config, window)?
        else {
            return Ok(false);
        };

        self.surface = surface.into();

        self.drawer = WGPUDrawer::default().into();

        Ok(true)
    }

    async fn start_internal(app: Box<dyn App>, event_loop: EventLoop<Events>) -> Result<()> {
        let instance = Instance::new(InstanceDescriptor {
            backends: Backends::all(),
            ..Default::default()
        });

        let adapter = instance
            .request_adapter(&RequestAdapterOptions::default())
            .await
            .ok_or(anyhow!("Failed to request adapter"))?;

        let info = adapter.get_info();

        info!("{}", &info.backend);

        let mut required_limits = if cfg!(target_arch = "wasm32") {
            Limits::downlevel_webgl2_defaults()
        } else {
            Limits::default()
        };

        required_limits.max_compute_workgroups_per_dimension = 65535;

        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    // required_features: Features::empty(),
                    required_features: Features::POLYGON_MODE_LINE, // | Features::POLYGON_MODE_POINT,
                    required_limits,
                    label: None,
                },
                None,
            )
            .await?;

        let config = SurfaceConfiguration {
            usage:        TextureUsages::RENDER_ATTACHMENT | TextureUsages::COPY_SRC,
            format:       TEXTURE_FORMAT,
            width:        1000,
            height:       1000,
            present_mode: if ENABLE_VSYNC || Platform::MOBILE {
                PresentMode::AutoVsync
            } else {
                PresentMode::AutoNoVsync
            },
            alpha_mode:   CompositeAlphaMode::Auto,
            view_formats: vec![],

            desired_maximum_frame_latency: 2,
        };

        let state = State::new(app);

        assert!(APP.is_none(), "Another instance of App already exists.");

        *APP.get_mut() = Self {
            state,
            window_size: Size::default(),
            config,
            device,
            queue,
            adapter,
            instance,
            resumed: false,
            surface: None,
            drawer: None,
            close: AtomicBool::default(),
        }
        .into();

        let app = Self::current();

        app.state.app.set_wgpu_app(Rglica::from_ref(app));
        app.start_event_loop(event_loop)
    }

    #[cfg(not(target_os = "android"))]
    pub async fn start(app: Box<dyn App>) -> Result<()> {
        Self::start_internal(app, EventLoop::new()?).await
    }

    #[cfg(target_os = "android")]
    pub async fn start(app: Box<dyn App>, event_loop: EventLoop<Events>) -> Result<()> {
        Self::start_internal(app, event_loop).await
    }

    fn start_event_loop(&mut self, event_loop: EventLoop<Events>) -> Result<()> {
        event_loop.run_app(self)?;
        Ok(())
    }

    pub fn set_title(&self, title: impl Into<String>) {
        if Platform::DESKTOP {
            Self::window().set_title(&title.into());
        }
    }

    pub fn set_window_size(&self, size: impl Into<Size<u32>>) {
        let size = size.into();
        let _ = Self::window().request_inner_size(PhysicalSize::new(size.width, size.height));
    }

    pub fn request_read_display(&self) -> Receiver<Screenshot> {
        self.state.request_read_display()
    }

    pub fn path_layout() -> &'static BindGroupLayout {
        &Self::drawer().path.color_size_layout
    }

    pub fn fps(&self) -> f32 {
        self.state.frame_counter.fps
    }

    pub fn frame_time(&self) -> f32 {
        self.state.frame_counter.frame_time
    }

    pub fn frame_drawn(&self) -> u32 {
        self.state.frame_counter.frame_count
    }

    pub fn display_refresh_rate() -> u32 {
        Self::window().current_monitor().map_or(60, |monitor| {
            monitor.refresh_rate_millihertz().unwrap_or(60_000) / 1000
        })
    }
}

impl ApplicationHandler<Events> for WGPUApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.create_surface_and_window(event_loop).unwrap() {
            self.state.app.window_ready();
        }
        self.resumed = true;
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
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
                    self.state.app.mouse_scroll(point * 28.0);
                }
                MouseScrollDelta::PixelDelta(delta) => {
                    self.state.app.mouse_scroll((delta.x, delta.y).into());
                }
            },
            WindowEvent::KeyboardInput { event, .. } => {
                if event.physical_key == PhysicalKey::Code(KeyCode::Escape) {
                    event_loop.exit();
                }
                self.state.app.key_event(event);
            }
            WindowEvent::DroppedFile(path) => {
                self.state.app.dropped_file(path);
            }
            WindowEvent::Resized(physical_size) => {
                self.state.resize(physical_size, event_loop);
            }
            WindowEvent::ScaleFactorChanged {
                scale_factor,
                inner_size_writer: _,
            } => {
                dbg!(&scale_factor);
            }
            WindowEvent::RedrawRequested => {
                if self.close.load(Ordering::Relaxed) {
                    event_loop.exit();
                }

                self.state.update();

                match self.state.render() {
                    Ok(()) => {}
                    Err(e) => error!("Render error: {e:?}"),
                };
            }
            _ => {}
        };
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if self.resumed {
            Self::window().request_redraw();
        }
    }
}
