#![feature(const_trait_impl)]
#![feature(effects)]

use std::sync::Arc;

use anyhow::Result;
use wgpu_wrapper::state::State;
use winit::{
    dpi::PhysicalSize,
    event::*,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::WindowBuilder,
};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let event_loop = EventLoop::new()?;
    let window = Arc::new(
        WindowBuilder::new()
            .with_inner_size(PhysicalSize::new(1200, 1200))
            .build(&event_loop)
            .unwrap(),
    );

    let mut state = State::new(window.clone()).await?;

    event_loop.run(move |event, elwt| match event {
        Event::WindowEvent { ref event, window_id } if window_id == window.id() => match event {
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
                state.resize(*physical_size);
                window.request_redraw();
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
                state.update();
                match state.render() {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                    Err(wgpu::SurfaceError::OutOfMemory) => elwt.exit(),
                    Err(e) => eprintln!("{e:?}"),
                }
            }
            _ => {}
        },
        _ => {}
    })?;

    Ok(())
}
