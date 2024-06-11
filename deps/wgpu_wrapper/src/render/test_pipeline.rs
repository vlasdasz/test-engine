use std::ops::Range;

use bytemuck::{bytes_of, Pod, Zeroable};
use gm::{checked_usize_to_u32, flat::Point, volume::Vertex};
use wgpu::{
    BindGroup, Buffer, BufferUsages, PipelineLayoutDescriptor, PolygonMode, PrimitiveTopology, RenderPass,
    RenderPipeline, ShaderStages, TextureFormat,
};

use crate::{
    image::Image,
    render::{uniform::make_uniform_layout, vertex_layout::VertexLayout},
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
    pos:      Point<f32>,
    z:        f32,
    _padding: u32,
}

#[derive(Debug)]
pub struct TestPipeline {
    render_pipeline: RenderPipeline,
    vertex_buffer:   Buffer,
    view_buffer:     Buffer,
    view_bind:       BindGroup,
    view:            BackgroundView,
}

impl TestPipeline {
    pub fn new(texture_format: TextureFormat) -> Self {
        let device = WGPUApp::device();
        let shader = device.create_shader_module(wgpu::include_wgsl!("shaders/test.wgsl"));

        let vertex_layout = make_uniform_layout("background_drawer_vertex_layout", ShaderStages::VERTEX);

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label:                "background_pipeline_layout".into(),
            bind_group_layouts:   &[&vertex_layout, &Image::uniform_layout()],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.pipeline(
            "background_pipeline",
            Some(&pipeline_layout),
            &shader,
            texture_format,
            PolygonMode::Fill,
            PrimitiveTopology::TriangleStrip,
            &[Vertex::VERTEX_LAYOUT],
        );

        let view = BackgroundView {
            pos:      Point::default(),
            z:        0.0,
            _padding: 0,
        };

        let vertex_buffer = device.buffer(&VERTICES, BufferUsages::VERTEX);
        let view_buffer = device.buffer(&view, BufferUsages::UNIFORM | BufferUsages::COPY_DST);

        let view_bind = device.bind(&view_buffer, &vertex_layout);

        Self {
            render_pipeline,
            vertex_buffer,
            view_buffer,
            view_bind,
            view,
        }
    }

    pub fn draw<'a>(
        &'a mut self,
        render_pass: &mut RenderPass<'a>,
        image: &'static Image,
        pos: Point,
        z: f32,
    ) {
        render_pass.set_pipeline(&self.render_pipeline);

        let view = BackgroundView { pos, z, _padding: 0 };

        if view != self.view {
            self.view = view;
            WGPUApp::queue().write_buffer(&self.view_buffer, 0, bytes_of(&view));
        }

        render_pass.set_bind_group(0, &self.view_bind, &[]);
        render_pass.set_bind_group(1, &image.bind, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.draw(RANGE, 0..1);
    }
}
