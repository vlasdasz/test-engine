use std::{
    mem::uninitialized,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
};

use anyhow::Result;
use gm::{
    LossyConvert,
    color::Color,
    flat::{Point, Size},
};
use log::{debug, error, info, warn};
use plat::Platform;
use refs::{Rglica, main_lock::MainLock};
use wgpu::{
    Adapter, Backends, CompositeAlphaMode, Device, DeviceDescriptor, Features, Instance, InstanceDescriptor,
    Limits, MemoryHints, PresentMode, Queue, RequestAdapterOptions, SurfaceConfiguration, TextureUsages,
    Trace,
};
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::{MouseScrollDelta, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{WindowAttributes, WindowId},
};

use crate::{
    Screenshot,
    state::{RGBA_TEXTURE_FORMAT, State},
    surface::Surface,
    window_events::WindowEvents,
};

const ENABLE_VSYNC: bool = true;
/// Doesn't work on some Androids
pub(crate) const SUPPORT_SCREENSHOT: bool = !Platform::ANDROID;

static WINDOW: MainLock<Option<Window>> = MainLock::new();

#[cfg(target_os = "android")]
pub type Events = winit::platform::android::activity::AndroidApp;

#[cfg(not(target_os = "android"))]
pub type Events = ();

pub struct Window {
    pub state: State,

    pub(crate) config: SurfaceConfiguration,
    pub(crate) device: Device,
    pub(crate) queue:  Queue,

    adapter:  Adapter,
    instance: Instance,

    pub(crate) resumed: bool,

    pub(crate) surface: Option<Surface>,

    pub(crate) title_set: bool,

    close: AtomicBool,

    initial_size: Size,
}

impl Window {
    pub fn current() -> &'static mut Self {
        WINDOW.get_mut().as_mut().expect("Window has not been initialized yet.")
    }

    pub fn device() -> &'static Device {
        &Self::current().device
    }

    pub fn queue() -> &'static Queue {
        &Self::current().queue
    }

    pub(crate) fn winit_window() -> &'static winit::window::Window {
        &Self::current()
            .surface
            .as_ref()
            .expect("Surface has not been initialized yet.")
            .window
    }

    pub fn inner_size() -> Size {
        let size = Self::winit_window().inner_size();
        (size.width, size.height).into()
    }

    pub fn outer_size() -> Size {
        let size = Self::winit_window().outer_size();
        (size.width, size.height).into()
    }

    pub fn render_size() -> Size {
        if Platform::IOS {
            Window::outer_size()
        } else {
            Window::inner_size()
        }
    }

    pub fn inner_position() -> Point {
        let pos = Self::winit_window().inner_position().unwrap_or_default();
        (pos.x, pos.y).into()
    }

    pub fn outer_position() -> Point {
        let pos = Self::winit_window().outer_position().unwrap_or_default();
        (pos.x, pos.y).into()
    }

    pub fn screen_scale() -> f32 {
        Self::winit_window().scale_factor().lossy_convert()
    }

    pub fn set_clear_color(color: impl Into<Color>) {
        Self::current().state.clear_color = color.into();
    }

    pub fn close() {
        // on_main(|| {
        //     Self::current().close.store(true, Ordering::Relaxed);
        // });
    }

    pub(crate) fn create_surface_and_window(
        &mut self,
        size: Size,
        event_loop: &ActiveEventLoop,
    ) -> Result<bool> {
        let window =
            Arc::new(event_loop.create_window(WindowAttributes::default().with_title("Test Engine"))?);

        let scale: f32 = window.scale_factor().lossy_convert();

        _ = window.request_inner_size(PhysicalSize::new(size.width * scale, size.height * scale));

        self.config.width = (size.width * scale).lossy_convert();
        self.config.height = (size.height * scale).lossy_convert();

        let Some(surface) = Surface::new(&self.instance, &self.adapter, &self.device, &self.config, window)?
        else {
            return Ok(false);
        };

        self.surface = surface.into();

        Ok(true)
    }

    fn start_internal(size: Size, app: Box<dyn WindowEvents>, event_loop: EventLoop<Events>) -> Result<()> {
        let instance = Instance::new(&InstanceDescriptor {
            backends: Backends::all(),
            ..Default::default()
        });

        let adapter: Arc<Mutex<Option<Adapter>>> = Arc::new(Mutex::new(None));

        let adapter_write = adapter.clone();
        let int = instance.clone();

        #[cfg(target_arch = "wasm32")]
        wasm_bindgen_futures::spawn_local(async move {
            let mut ad = adapter_write.lock().unwrap();

            *ad = Some(int.request_adapter(&RequestAdapterOptions::default()).await.unwrap());

            drop(ad);
        });

        #[cfg(not(target_arch = "wasm32"))]
        pollster::block_on(async move {
            let mut ad = adapter_write.lock().unwrap();

            *ad = Some(int.request_adapter(&RequestAdapterOptions::default()).await.unwrap());
            drop(ad);
        });

        let adapter = adapter.lock().unwrap().take().unwrap();
        let ad2 = adapter.clone();

        let info = adapter.get_info();

        info!("Backend: {}", &info.backend);

        let mut required_limits = if cfg!(target_arch = "wasm32") {
            Limits::downlevel_webgl2_defaults()
        } else {
            Limits::default()
        };

        if Platform::IOS {
            required_limits.max_color_attachments = 4;
        } else if Platform::ANDROID {
            // TODO:
            required_limits.max_compute_invocations_per_workgroup = 0;
            required_limits.max_compute_workgroups_per_dimension = 0;
            required_limits.max_compute_workgroup_storage_size = 0;
            required_limits.max_compute_workgroup_size_x = 0;
            required_limits.max_compute_workgroup_size_y = 0;
            required_limits.max_compute_workgroup_size_z = 0;
            required_limits.max_storage_buffer_binding_size = 0;
            required_limits.max_storage_textures_per_shader_stage = 0;
            required_limits.max_storage_buffers_per_shader_stage = 0;
            required_limits.max_dynamic_storage_buffers_per_pipeline_layout = 0;
            required_limits.max_texture_dimension_3d = 1024;
            required_limits.max_texture_dimension_2d = 4096;
            required_limits.max_texture_dimension_1d = 4096;
        }

        let dq: Arc<Mutex<Option<_>>> = Arc::new(Mutex::new(None));
        let dq2 = dq.clone();

        #[cfg(target_arch = "wasm32")]
        wasm_bindgen_futures::spawn_local(async move {
            let mut dq = dq2.lock().unwrap();

            *dq = ad2
                .request_device(&DeviceDescriptor {
                    required_features: Features::empty(),
                    // Doesn't work on some Androids
                    // required_features: Features::POLYGON_MODE_LINE, // | Features::POLYGON_MODE_POINT,
                    required_limits,
                    label: None,
                    memory_hints: MemoryHints::Performance,
                    trace: Trace::default(),
                })
                .await
                .unwrap()
                .into();
        });

        #[cfg(not(target_arch = "wasm32"))]
        pollster::block_on(async move {
            let mut dq = dq2.lock().unwrap();

            *dq = ad2
                .request_device(&DeviceDescriptor {
                    required_features: Features::empty(),
                    // Doesn't work on some Androids
                    // required_features: Features::POLYGON_MODE_LINE, // | Features::POLYGON_MODE_POINT,
                    required_limits,
                    label: None,
                    memory_hints: MemoryHints::Performance,
                    trace: Trace::default(),
                })
                .await
                .unwrap()
                .into();
        });

        let (device, queue) = dq.lock().unwrap().take().unwrap();

        let config = SurfaceConfiguration {
            usage:        if SUPPORT_SCREENSHOT {
                TextureUsages::RENDER_ATTACHMENT | TextureUsages::COPY_SRC
            } else {
                TextureUsages::RENDER_ATTACHMENT
            },
            format:       RGBA_TEXTURE_FORMAT,
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

        assert!(WINDOW.is_none(), "Another instance of Window already exists.");

        *WINDOW.get_mut() = Self {
            state,
            config,
            device,
            queue,
            adapter,
            instance,
            resumed: false,
            surface: None,
            title_set: false,
            close: AtomicBool::default(),
            initial_size: size,
        }
        .into();

        let window = Self::current();

        window.state.app.set_window(Rglica::from_ref(window));
        window.start_event_loop(event_loop)
    }

    #[cfg(not(target_os = "android"))]
    pub fn start(size: Size, app: impl WindowEvents + 'static) -> Result<()> {
        Self::start_internal(size, Box::new(app), EventLoop::new()?)
    }

    #[cfg(target_os = "android")]
    pub async fn start(app: impl WindowEvents + 'static, event_loop: EventLoop<Events>) -> Result<()> {
        Self::start_internal((1200, 1000).into(), Box::new(app), event_loop).await
    }

    fn start_event_loop(&mut self, event_loop: EventLoop<Events>) -> Result<()> {
        event_loop.run_app(self)?;
        Ok(())
    }

    pub fn set_title(title: impl Into<String>) {
        // let title = title.into();
        // on_main(move || {
        //     Self::current().title_set = true;
        //     if Platform::DESKTOP {
        //         Self::winit_window().set_title(&title);
        //     } else {
        //         warn!("set_title is not supported on this platform");
        //     }
        // });
    }

    pub fn set_size(&self, size: impl Into<Size<u32>>) {
        let size = size.into();
        let _ = Self::winit_window().request_inner_size(PhysicalSize::new(size.width, size.height));
    }

    // pub fn request_screenshot(&self) -> Receiver<Screenshot> {
    //     self.state.request_read_display()
    // }

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
        Self::winit_window().current_monitor().map_or(60, |monitor| {
            monitor.refresh_rate_millihertz().unwrap_or(60_000) / 1000
        })
    }
}

impl ApplicationHandler<Events> for Window {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.create_surface_and_window(self.initial_size, event_loop).unwrap() {
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
                debug!("Scale factor: {scale_factor}");
            }
            WindowEvent::RedrawRequested => {
                if self.close.load(Ordering::Relaxed) {
                    event_loop.exit();
                }

                self.state.update();

                match self.state.render() {
                    Ok(()) => {}
                    Err(e) => error!("Render error: {e:?}"),
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if self.resumed {
            Self::winit_window().request_redraw();
        }
    }
}
