use std::ops::Range;

use gm::{
    checked_usize_to_u32,
    flat::{Point, Rect, Vertex2D},
};
use wgpu::{
    BindGroupLayout, Buffer, BufferUsages, PolygonMode, PrimitiveTopology, RenderPipeline, ShaderStages,
};

use crate::{
    Window,
    image::Image,
    render::{
        uniform::{cached_float_bind, make_uniform_layout},
        vertex_layout::VertexLayout,
    },
    utils::DeviceHelper,
};

pub const fn image_vertices_with_shrink(x: f32, y: f32, width: f32, height: f32) -> [Vertex2D; 4] {
    [
        Vertex2D {
            pos: Point::new(-1.0, 1.0),
            uv:  Point::new(0.0 + x, 0.0 + y),
        },
        Vertex2D {
            pos: Point::new(-1.0, -1.0),
            uv:  Point::new(0.0 + x, 1.0 * height + y),
        },
        Vertex2D {
            pos: Point::new(1.0, 1.0),
            uv:  Point::new(1.0 * width + x, 0.0 + y),
        },
        Vertex2D {
            pos: Point::new(1.0, -1.0),
            uv:  Point::new(1.0 * width + x, 1.0 * height + y),
        },
    ]
}

const VERTICES: [Vertex2D; 4] = image_vertices_with_shrink(0.0, 0.0, 1.0, 1.0);

const RANGE: Range<u32> = 0..checked_usize_to_u32(VERTICES.len());

#[derive(Debug)]
pub struct ImageDrawer {
    render_pipeline: RenderPipeline,
    vertex_buffer:   Buffer,
    vertex_layout:   BindGroupLayout,
}

impl Default for ImageDrawer {
    fn default() -> Self {
        let device = Window::device();
        let shader = device.create_shader_module(wgpu::include_wgsl!("shaders/ui_image.wgsl"));

        let vertex_layout = make_uniform_layout("image_drawer_vertex_layout", ShaderStages::VERTEX);

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label:                "Colored Image Pipeline Layout".into(),
            bind_group_layouts:   &[&vertex_layout, &Image::uniform_layout()],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.pipeline(
            "Colored Image Render Pipeline",
            &pipeline_layout,
            &shader,
            PolygonMode::Fill,
            PrimitiveTopology::TriangleStrip,
            &[Vertex2D::VERTEX_LAYOUT],
        );

        Self {
            render_pipeline,
            vertex_buffer: device.buffer(&VERTICES, BufferUsages::VERTEX),
            vertex_layout,
        }
    }
}

impl ImageDrawer {
    pub fn draw<'a>(
        &'a self,
        render_pass: &mut wgpu::RenderPass<'a>,
        image: &'static Image,
        rect: &Rect,
        cropped_vertices: Option<&'a Buffer>,
        z_position: f32,
    ) {
        render_pass.set_viewport(rect.x(), rect.y(), rect.width(), rect.height(), 0., 1.);
        render_pass.set_pipeline(&self.render_pipeline);

        render_pass.set_bind_group(0, cached_float_bind(z_position, &self.vertex_layout), &[]);
        render_pass.set_bind_group(1, &image.bind, &[]);
        render_pass.set_vertex_buffer(0, cropped_vertices.unwrap_or(&self.vertex_buffer).slice(..));
        render_pass.draw(RANGE, 0..1);
    }
}
