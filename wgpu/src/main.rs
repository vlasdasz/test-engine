#![feature(const_trait_impl)]
#![feature(effects)]

// mod rect_state;
mod image_state;
mod rect_state;
mod state;
pub mod texture;

use std::{mem::size_of, sync::Arc};

use anyhow::Result;
use gm::{flat::Point, volume::UIVertex};
use winit::{
    event::*,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::WindowBuilder,
};

use crate::state::State;

pub trait VertexLayout: Sized {
    const ATTRIBS: &'static [wgpu::VertexAttribute];
    fn vertex_layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: size_of::<Self>() as wgpu::BufferAddress,
            step_mode:    wgpu::VertexStepMode::Vertex,
            attributes:   Self::ATTRIBS,
        }
    }
}

impl VertexLayout for Point {
    const ATTRIBS: &'static [wgpu::VertexAttribute] = &wgpu::vertex_attr_array![0 => Float32x2];
}

impl VertexLayout for UIVertex {
    const ATTRIBS: &'static [wgpu::VertexAttribute] =
        &wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x2];
}

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
