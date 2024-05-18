use std::ops::Range;

use bytemuck::bytes_of;
use gm::{
    checked_usize_to_u32,
    flat::{Point, Rect},
    volume::UIVertex,
};
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BindGroupLayout, Buffer, BufferUsages, PolygonMode, RenderPipeline, ShaderStages, TextureFormat,
};

use crate::{
    image::Image,
    render::uniform::{make_bind, make_layout},
    utils::make_pipeline,
    WGPUApp,
};

pub const fn image_vertices_with_shrink(x: f32, y: f32, width: f32, height: f32) -> [UIVertex; 4] {
    [
        UIVertex {
            pos: Point::new(-1., 1.),
            uv:  Point::new(0. + x, 0. + y),
        },
        UIVertex {
            pos: Point::new(-1., -1.),
            uv:  Point::new(0. + x, 1. * height + y),
        },
        UIVertex {
            pos: Point::new(1., 1.),
            uv:  Point::new(1. * width + x, 0. + y),
        },
        UIVertex {
            pos: Point::new(1., -1.),
            uv:  Point::new(1. * width + x, 1. * height + y),
        },
    ]
}

const VERTICES: [UIVertex; 4] = image_vertices_with_shrink(0., 0., 1., 1.);

const RANGE: Range<u32> = 0..checked_usize_to_u32(VERTICES.len());

#[derive(Debug)]
pub struct ImageDrawer {
    render_pipeline: RenderPipeline,
    vertex_buffer:   Buffer,
    vertex_layout:   BindGroupLayout,
}

impl ImageDrawer {
    pub fn new() -> Self {
        let device = WGPUApp::device();
        let shader = device.create_shader_module(wgpu::include_wgsl!("shaders/ui_image.wgsl"));

        let vertex_layout = make_layout("image_drawer_vertex_layout", ShaderStages::VERTEX, 1);

        let image_layout = Image::bind_group_layout();

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label:                "Colored Image Pipeline Layout".into(),
            bind_group_layouts:   &[&vertex_layout, &image_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = make_pipeline::<UIVertex>(
            "Colored Image Render Pipeline",
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
            vertex_layout,
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
        render_pass.set_viewport(rect.x(), rect.y(), rect.width(), rect.height(), 0., 1.);
        render_pass.set_pipeline(&self.render_pipeline);

        render_pass.set_bind_group(0, make_bind([bytes_of(&z_position)], &self.vertex_layout), &[]);
        render_pass.set_bind_group(1, &image.bind, &[]);
        render_pass.set_vertex_buffer(0, vertices.unwrap_or(&self.vertex_buffer).slice(..));
        render_pass.draw(RANGE, 0..1);
    }
}
