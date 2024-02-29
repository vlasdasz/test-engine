use std::ops::Range;

use gm::{
    checked_usize_to_u32,
    flat::{Point, Rect},
    volume::UIVertex,
};
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BindGroupLayout, Buffer, BufferUsages, PolygonMode, RenderPipeline, TextureFormat,
};

use crate::{image::Image, render::uniform::OldUniform, utils::make_pipeline, WGPUApp};

pub const fn image_vertices_with_shrink(x: f32, y: f32, width: f32, height: f32) -> [UIVertex; 4] {
    [
        UIVertex {
            pos: Point::new(-1.0, 1.0),
            uv:  Point::new(0.0 + x, 0.0 + y),
        },
        UIVertex {
            pos: Point::new(-1.0, -1.0),
            uv:  Point::new(0.0 + x, 1.0 * height + y),
        },
        UIVertex {
            pos: Point::new(1.0, 1.0),
            uv:  Point::new(1.0 * width + x, 0.0 + y),
        },
        UIVertex {
            pos: Point::new(1.0, -1.0),
            uv:  Point::new(1.0 * width + x, 1.0 * height + y),
        },
    ]
}

const VERTICES: [UIVertex; 4] = image_vertices_with_shrink(0.0, 0.0, 1.0, 1.0);

const RANGE: Range<u32> = 0..checked_usize_to_u32(VERTICES.len());

#[derive(Debug)]
pub struct ImageState {
    render_pipeline: RenderPipeline,
    vertex_buffer:   Buffer,
    z_layout:        BindGroupLayout,
}

impl ImageState {
    pub fn new() -> Self {
        let device = WGPUApp::device();
        let shader = device.create_shader_module(wgpu::include_wgsl!("shaders/ui_image.wgsl"));

        let z_layout = OldUniform::z_layout();
        let image_layout = Image::bind_group_layout();

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label:                "Colored Image Pipeline Layout".into(),
            bind_group_layouts:   &[&z_layout, &image_layout],
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
            contents: bytemuck::cast_slice(&VERTICES),
            usage:    BufferUsages::VERTEX,
        });

        Self {
            render_pipeline,
            vertex_buffer,
            z_layout,
        }
    }

    pub fn draw<'a>(
        &'a self,
        image: &'static Image,
        rect: &Rect,
        render_pass: &mut wgpu::RenderPass<'a>,
        vertices: Option<&'a Buffer>,
        z_position: f32,
    ) {
        render_pass.set_viewport(rect.x(), rect.y(), rect.width(), rect.height(), 0.0, 1.0);
        render_pass.set_pipeline(&self.render_pipeline);

        render_pass.set_bind_group(0, OldUniform::z(&self.z_layout, z_position), &[]);
        render_pass.set_bind_group(1, &image.bind, &[]);
        render_pass.set_vertex_buffer(0, vertices.unwrap_or(&self.vertex_buffer).slice(..));
        render_pass.draw(RANGE, 0..1);
    }
}
