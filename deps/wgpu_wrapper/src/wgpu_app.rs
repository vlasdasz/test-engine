use std::sync::Arc;

use anyhow::Result;
use log::error;
use refs::MainLock;
use winit::{
    dpi::PhysicalSize,
    event::{Event, KeyEvent, WindowEvent},
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowBuilder},
};

use crate::{app::App, state::State};

static APP: MainLock<Option<WGPUApp>> = MainLock::new();

pub struct WGPUApp {
    pub(crate) state:  State,
    pub(crate) window: Arc<Window>,
    event_loop:        Option<EventLoop<()>>,
}

impl WGPUApp {
    pub(crate) fn current() -> &'static mut Self {
        APP.get_mut().as_mut().expect("App has not been initialized yet.")
    }

    pub async fn start(app: impl App + 'static, width: u32, height: u32) -> Result<()> {
        env_logger::init();
        let event_loop = EventLoop::new()?;
        let window = Arc::new(
            WindowBuilder::new()
                .with_title("Test Engine")
                .with_inner_size(PhysicalSize::new(width, height))
                .build(&event_loop)
                .unwrap(),
        );

        let state = State::new(app, window.clone()).await?;

        assert!(APP.is_none(), "Another instance of App already exists.");

        *APP.get_mut() = Self {
            state,
            window,
            event_loop: event_loop.into(),
        }
        .into();

        APP.get_mut().as_mut().unwrap().start_event_loop()
    }

    fn start_event_loop(&mut self) -> Result<()> {
        self.event_loop.take().unwrap().run(move |event, elwt| match event {
            Event::WindowEvent { ref event, window_id } if window_id == self.window.id() => match event {
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            physical_key: PhysicalKey::Code(KeyCode::Escape),
                            ..
                        },
                    ..
                } => elwt.exit(),
                WindowEvent::Resized(physical_size) => {
                    self.state.resize(*physical_size);
                    self.window.request_redraw();
                }
                WindowEvent::ScaleFactorChanged {
                    scale_factor,
                    inner_size_writer,
                } => {
                    dbg!(&scale_factor);
                    dbg!(&inner_size_writer);
                    //state.resize(**new_inner_size);
                }
                WindowEvent::RedrawRequested => {
                    self.state.update();
                    self.state.app.update();
                    match self.state.render() {
                        Ok(()) => {}
                        // Err(wgpu::SurfaceError::Lost) => self.state.resize(self.state.size),
                        // Err(wgpu::SurfaceError::OutOfMemory) => elwt.exit(),
                        Err(e) => error!("{e:?}"),
                    }
                }
                _ => {}
            },
            _ => {}
        })?;

        Ok(())
    }
}
