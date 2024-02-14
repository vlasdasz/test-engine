use std::ops::Range;

use gm::{
    flat::{Point, Rect},
    volume::UIVertex,
};
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    PolygonMode, TextureFormat,
};

use crate::{image::Image, utils::make_pipeline};

const VERTICES: &[UIVertex] = &[
    UIVertex {
        pos: Point::new(-1.0, 1.0),
        uv:  Point::new(0.0, 0.0),
    },
    UIVertex {
        pos: Point::new(-1.0, -1.0),
        uv:  Point::new(0.0, 1.0),
    },
    UIVertex {
        pos: Point::new(1.0, 1.0),
        uv:  Point::new(1.0, 0.0),
    },
    UIVertex {
        pos: Point::new(1.0, -1.0),
        uv:  Point::new(1.0, 1.0),
    },
];

#[allow(clippy::cast_possible_truncation)]
const RANGE: Range<u32> = 0..(VERTICES.len() as u32);

#[derive(Debug)]
pub struct ImageState {
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer:   wgpu::Buffer,
}

impl ImageState {
    pub fn new(device: &wgpu::Device) -> Self {
        let shader = device.create_shader_module(wgpu::include_wgsl!("../shaders/ui_image.wgsl"));

        let bind_group_layout = Image::bind_group_layout(device);

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label:                Some("Colored Image Pipeline Layout"),
            bind_group_layouts:   &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = make_pipeline::<UIVertex>(
            "Colored Image Render Pipeline",
            device,
            &pipeline_layout,
            &shader,
            TextureFormat::Bgra8UnormSrgb,
            PolygonMode::Fill,
        );

        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label:    "Colored Image Vertex Buffer".into(),
            contents: bytemuck::cast_slice(VERTICES),
            usage:    wgpu::BufferUsages::VERTEX,
        });

        Self {
            render_pipeline,
            vertex_buffer,
        }
    }

    pub fn draw<'a>(&'a self, image: &'static Image, rect: &Rect, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_viewport(rect.x(), rect.y(), rect.width(), rect.height(), 0.0, 1.0);
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &image.bind, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.draw(RANGE, 0..1);
    }
}
