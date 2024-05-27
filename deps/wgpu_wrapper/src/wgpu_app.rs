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
    Adapter, BindGroupLayout, CompositeAlphaMode, Device, Instance, PresentMode, Queue, SurfaceConfiguration,
};
use winit::{
    dpi::PhysicalSize,
    event::{Event, MouseScrollDelta, WindowEvent},
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowBuilder},
};

use crate::{
    app::App,
    state::{State, TEXTURE_FORMAT},
    surface::Surface,
    Screenshot, WGPUDrawer,
};

static APP: MainLock<Option<WGPUApp>> = MainLock::new();

#[cfg(target_os = "android")]
pub type Events = winit::platform::android::activity::AndroidApp;

#[cfg(not(target_os = "android"))]
pub type Events = ();

pub struct WGPUApp {
    pub state:         State,
    pub(crate) window: Arc<Window>,

    pub window_size: Size,

    pub(crate) config: SurfaceConfiguration,
    pub(crate) device: Device,
    pub(crate) queue:  Queue,

    adapter:  Adapter,
    instance: Instance,

    pub(crate) resumed: bool,

    pub(crate) surface: Option<Surface>,
    pub(crate) drawer:  Option<WGPUDrawer>,

    event_loop: Option<EventLoop<Events>>,
    close:      AtomicBool,
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

    pub fn drawer() -> &'static mut WGPUDrawer {
        Self::current().drawer.as_mut().expect("Drawer has not been initialized yet.")
    }

    pub fn screen_scale() -> f64 {
        Self::current().window.scale_factor()
    }

    pub fn close() {
        on_main(|| {
            Self::current().close.store(true, Ordering::Relaxed);
        });
    }

    pub(crate) fn create_surface(&mut self) -> Result<bool> {
        let Some(surface) = Surface::new(
            &self.instance,
            &self.adapter,
            &self.device,
            &self.config,
            self.window.clone(),
        )?
        else {
            return Ok(false);
        };

        self.surface = surface.into();

        self.drawer = WGPUDrawer::new()?.into();

        Ok(true)
    }

    async fn start_internal(app: Box<dyn App>, event_loop: EventLoop<Events>) -> Result<()> {
        let window = Arc::new(WindowBuilder::new().with_title("Test Engine").build(&event_loop)?);

        let scale: u32 = window.scale_factor().lossy_convert();

        _ = window.request_inner_size(PhysicalSize::new(1200 * scale, 1000 * scale));

        let instance = Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await
            .ok_or(anyhow!("Failed to request adapter"))?;

        let info = adapter.get_info();

        info!("{}", &info.backend);

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    required_features: wgpu::Features::empty(), //wgpu::Features::POLYGON_MODE_LINE,
                    required_limits:   if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                    label:             None,
                },
                None,
            )
            .await?;

        let size = window.inner_size();

        let config = SurfaceConfiguration {
            usage:        wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
            format:       TEXTURE_FORMAT,
            width:        size.width,
            height:       size.height,
            present_mode: if Platform::MOBILE {
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
            window,
            window_size: Default::default(),
            config,
            device,
            queue,
            adapter,
            instance,
            resumed: false,
            surface: None,
            drawer: None,
            event_loop: event_loop.into(),
            close: Default::default(),
        }
        .into();

        let app = Self::current();

        app.state.app.set_wgpu_app(Rglica::from_ref(app));
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
            Event::WindowEvent { event, window_id } if window_id == self.window.id() => match event {
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
                self.window.request_redraw();
            }
            Event::Resumed => {
                if self.create_surface().unwrap() {
                    self.state.app.window_ready();
                }
                self.resumed = true;
            }
            _ => {}
        })?;

        Ok(())
    }

    pub fn set_title(&self, title: impl ToString) {
        if Platform::DESKTOP {
            self.window.set_title(&title.to_string());
        }
    }

    pub fn set_window_size(&self, size: impl Into<Size<u32>>) {
        let size = size.into();
        let _ = self.window.request_inner_size(PhysicalSize::new(size.width, size.height));
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
