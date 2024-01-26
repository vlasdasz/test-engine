#![feature(const_trait_impl)]
#![feature(effects)]

mod state;

use std::sync::Arc;

use anyhow::Result;
use winit::{
    event::*,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::WindowBuilder,
};

use crate::state::State;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let event_loop = EventLoop::new()?;
    let window = Arc::new(WindowBuilder::new().build(&event_loop).unwrap());

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
                // new_inner_size is &&mut so we have to dereference it twice
                //state.resize(**new_inner_size);
            }
            WindowEvent::RedrawRequested => {
                state.update();
                match state.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if lost
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => elwt.exit(),
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{e:?}"),
                }
            }
            _ => {}
        },
        // Event::RedrawRequested(window_id) if window_id == window.id() => {

        // }
        // Event::MainEventsCleared => {
        //     // RedrawRequested will only trigger once unless we manually
        //     // request it.
        //     //state.window().request_redraw();
        // }
        _ => {}
    })?;

    Ok(())
}
