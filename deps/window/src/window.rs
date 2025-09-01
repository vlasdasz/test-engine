use std::{
    mem::uninitialized,
    rc::Rc,
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
use log::{debug, error, info};
use plat::Platform;
use refs::{Rglica, main_lock::MainLock};
use wgpu::{
    Adapter, Backends, CompositeAlphaMode, Device, DeviceDescriptor, Features, Instance, InstanceDescriptor,
    Limits, MemoryHints, PowerPreference, PresentMode, Queue, RequestAdapterOptions, SurfaceConfiguration,
    TextureUsages, Trace,
};
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::{MouseScrollDelta, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy},
    keyboard::{KeyCode, PhysicalKey},
    window::{WindowAttributes, WindowId},
};

use crate::{
    app_handler::AppHandler,
    state::{RGBA_TEXTURE_FORMAT, State},
    surface::Surface,
    window_events::WindowEvents,
};

const ENABLE_VSYNC: bool = true;
/// Doesn't work on some Androids and on Web
pub(crate) const SUPPORT_SCREENSHOT: bool = !Platform::ANDROID && !Platform::WASM;

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

    pub(crate) surface: Surface,

    pub(crate) title_set: bool,
}

impl Window {
    pub fn current() -> &'static mut Self {
        AppHandler::window()
    }

    pub fn device() -> &'static Device {
        &Self::current().device
    }

    pub fn queue() -> &'static Queue {
        &Self::current().queue
    }

    pub(crate) fn winit_window() -> &'static winit::window::Window {
        &mut Self::current().surface.window
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

    pub(crate) async fn start_internal(window: winit::window::Window, proxy: EventLoopProxy<Window>) {
        let window = Arc::new(window);

        let instance = Instance::default();
        let surface = instance.create_surface(window.clone()).unwrap();
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference:       PowerPreference::default(), // Power preference for the device
                force_fallback_adapter: false,                      /* Indicates that only a fallback
                                                                     * ("software") adapter can be used */
                compatible_surface:     Some(&surface), /* Guarantee that the adapter can render to this
                                                         * surface */
            })
            .await
            .expect("Could not get an adapter (GPU).");

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

        let (device, queue) = adapter
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
            .expect("AAAAAA");

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

        let state = State::new();

        let scale: f32 = window.scale_factor().lossy_convert();

        // self.config.width = (size.width * scale).lossy_convert();
        // self.config.height = (size.height * scale).lossy_convert();

        let surface =
            Surface::new(&instance, &adapter, &device, &config, window).expect("Failed to create surface");

        let window = Self {
            state,
            config,
            device,
            queue,
            adapter,
            instance,
            resumed: false,
            surface,
            title_set: false,
        };

        if proxy.send_event(window).is_err() {
            panic!("Failed to send window event")
        }
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
