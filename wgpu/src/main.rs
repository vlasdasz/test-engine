mod state;
pub mod texture;

use std::sync::Arc;

use anyhow::Result;
use winit::{
    event::*,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::WindowBuilder,
};

use crate::state::State;

// use crate::state::State;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position:   [f32; 3],
    tex_coords: [f32; 2], // NEW!
}

impl Vertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode:    wgpu::VertexStepMode::Vertex,
            attributes:   &[
                wgpu::VertexAttribute {
                    offset:          0,
                    shader_location: 0,
                    format:          wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset:          mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format:          wgpu::VertexFormat::Float32x2, // NEW!
                },
            ],
        }
    }
}

const VERTICES: &[Vertex] = &[
    // Changed
    Vertex {
        position:   [-0.0868241, 0.49240386, 0.0],
        tex_coords: [0.4131759, 0.00759614],
    }, // A
    Vertex {
        position:   [-0.49513406, 0.06958647, 0.0],
        tex_coords: [0.0048659444, 0.43041354],
    }, // B
    Vertex {
        position:   [-0.21918549, -0.44939706, 0.0],
        tex_coords: [0.28081453, 0.949397],
    }, // C
    Vertex {
        position:   [0.35966998, -0.3473291, 0.0],
        tex_coords: [0.85967, 0.84732914],
    }, // D
    Vertex {
        position:   [0.44147372, 0.2347359, 0.0],
        tex_coords: [0.9414737, 0.2652641],
    }, // E
];

const INDICES: &[u16] = &[0, 1, 4, 1, 2, 4, 2, 3, 4];

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let event_loop = EventLoop::new()?;
    let window = Arc::new(WindowBuilder::new().build(&event_loop).unwrap());

    let mut state = State::new(window.clone()).await;

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
