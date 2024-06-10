use std::ops::Range;

use bytemuck::{Pod, Zeroable};
use gm::{checked_usize_to_u32, flat::Point, volume::Vertex};
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BindGroupLayout, Buffer, BufferUsages, PolygonMode, PrimitiveTopology, RenderPipeline, ShaderStages,
    TextureFormat,
};

use crate::{
    image::Image,
    render::{
        uniform::{cached_z_bind, make_uniform_layout},
        vertex_layout::VertexLayout,
    },
    utils::DeviceHelper,
    WGPUApp,
};

const VAL: f32 = 0.8;

const fn image_vertices() -> [Vertex; 4] {
    [
        Vertex {
            pos: Point::new(-VAL, VAL),
            uv:  Point::new(-1.0, -1.0),
        },
        Vertex {
            pos: Point::new(-VAL, -VAL),
            uv:  Point::new(-1.0, VAL * 2.0),
        },
        Vertex {
            pos: Point::new(VAL, VAL),
            uv:  Point::new(VAL * 2.0, -1.0),
        },
        Vertex {
            pos: Point::new(VAL, -VAL),
            uv:  Point::new(VAL * 2.0, VAL * 2.0),
        },
    ]
}

const VERTICES: [Vertex; 4] = image_vertices();

const RANGE: Range<u32> = 0..checked_usize_to_u32(VERTICES.len());

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Zeroable, Pod, PartialEq)]
struct BackgroundView {
    pos: Point<f32>,
    z:   f32,
}

#[derive(Debug)]
pub struct TestPipeline {
    render_pipeline: RenderPipeline,
    vertex_buffer:   Buffer,
    vertex_layout:   BindGroupLayout,
    view_buffer:     Buffer,
}

impl TestPipeline {
    pub fn new(texture_format: TextureFormat) -> Self {
        let device = WGPUApp::device();
        let shader = device.create_shader_module(wgpu::include_wgsl!("shaders/test.wgsl"));

        let vertex_layout = make_uniform_layout("background_drawer_vertex_layout", ShaderStages::VERTEX);

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label:                "background_pipeline_layout".into(),
            bind_group_layouts:   &[&vertex_layout, &Image::uniform_layout()],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.make_pipeline(
            "background_pipeline",
            Some(&pipeline_layout),
            &shader,
            texture_format,
            PolygonMode::Fill,
            PrimitiveTopology::TriangleStrip,
            &[Vertex::VERTEX_LAYOUT],
        );

        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label:    "background_vertex_buffer".into(),
            contents: bytemuck::cast_slice(&VERTICES),
            usage:    BufferUsages::VERTEX,
        });

        let view_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label:    "background_view_buffer".into(),
            contents: bytemuck::bytes_of(&BackgroundView {
                pos: Default::default(),
                z:   0.0,
            }),
            usage:    BufferUsages::UNIFORM,
        });

        Self {
            render_pipeline,
            vertex_buffer,
            vertex_layout,
            view_buffer,
        }
    }

    pub fn draw<'a>(
        &'a self,
        image: &'static Image,
        render_pass: &mut wgpu::RenderPass<'a>,
        z_position: f32,
    ) {
        render_pass.set_pipeline(&self.render_pipeline);

        render_pass.set_bind_group(0, cached_z_bind(z_position, &self.vertex_layout), &[]);
        render_pass.set_bind_group(1, &image.bind, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.draw(RANGE, 0..1);
    }
}
