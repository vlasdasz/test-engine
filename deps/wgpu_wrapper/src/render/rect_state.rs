use std::{collections::HashMap, ops::Range};

use bytemuck::cast_slice;
use gm::{
    flat::{Point, Rect},
    Color,
};
use refs::MainLock;
use wgpu::{
    include_wgsl,
    util::{BufferInitDescriptor, DeviceExt},
    BindGroup, BindGroupEntry, BindGroupLayout, BindGroupLayoutEntry, BindingResource, BindingType, Buffer,
    BufferBinding, BufferBindingType, BufferUsages, Device, IndexFormat, PipelineLayoutDescriptor,
    PolygonMode, RenderPass, RenderPipeline, ShaderStages, TextureFormat,
};

use crate::utils::make_pipeline;

static BINDS: MainLock<HashMap<Color, BindGroup>> = MainLock::new();

const VERTICES: &[Point] = &[
    Point::new(-1.0, 1.0),
    Point::new(-1.0, -1.0),
    Point::new(1.0, 1.0),
    Point::new(1.0, -1.0),
];

const INDICES: &[u16] = &[0, 1, 2, 1, 2, 3];

#[allow(clippy::cast_possible_truncation)]
const VERTEX_RANGE: Range<u32> = 0..(VERTICES.len() as u32);

#[allow(clippy::cast_possible_truncation)]
const INDEX_RANGE: Range<u32> = 0..(INDICES.len() as u32);

#[derive(Debug)]
pub struct RectState {
    bind_group_layout: BindGroupLayout,
    fill_pipeline:     RenderPipeline,
    line_pipeline:     RenderPipeline,
    vertex_buffer:     Buffer,
    index_buffer:      Buffer,
}

impl RectState {
    pub fn new(device: &Device, texture_format: TextureFormat) -> Self {
        let shader = device.create_shader_module(include_wgsl!("shaders/rect.wgsl"));

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label:   Some("rect_bind_group_layout"),
            entries: &[BindGroupLayoutEntry {
                binding:    0,
                visibility: ShaderStages::FRAGMENT,
                ty:         BindingType::Buffer {
                    ty:                 BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size:   None,
                },
                count:      None,
            }],
        });

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label:                Some("Rect Pipeline Layout"),
            bind_group_layouts:   &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let fill_pipeline = make_pipeline::<Point>(
            "Rect Fill Render Pipeline",
            device,
            &pipeline_layout,
            &shader,
            texture_format,
            PolygonMode::Fill,
        );

        let line_pipeline = make_pipeline::<Point>(
            "Rect Line Render Pipeline",
            device,
            &pipeline_layout,
            &shader,
            texture_format,
            PolygonMode::Line,
        );

        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label:    Some("Rect Vertex Buffer"),
            contents: cast_slice(VERTICES),
            usage:    BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label:    Some("Index Buffer"),
            contents: cast_slice(INDICES),
            usage:    BufferUsages::INDEX,
        });

        Self {
            bind_group_layout,
            fill_pipeline,
            line_pipeline,
            vertex_buffer,
            index_buffer,
        }
    }

    fn bind_group_with_color(
        bind_group_layout: &BindGroupLayout,
        device: &Device,
        color: &Color,
    ) -> BindGroup {
        let color_uniform_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label:    Some("Color Uniform Buffer"),
            contents: cast_slice(&color.as_slice()),
            usage:    BufferUsages::UNIFORM,
        });

        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label:   Some("rect_bind_group"),
            layout:  bind_group_layout,
            entries: &[BindGroupEntry {
                binding:  0,
                resource: BindingResource::Buffer(BufferBinding {
                    buffer: &color_uniform_buffer,
                    offset: 0,
                    size:   None,
                }),
            }],
        })
    }

    fn pipeline(&self, polygon_mode: PolygonMode) -> &RenderPipeline {
        match polygon_mode {
            PolygonMode::Fill => &self.fill_pipeline,
            PolygonMode::Line => &self.line_pipeline,
            PolygonMode::Point => unimplemented!(),
        }
    }

    fn draw_vertices<'a>(&'a self, render_pass: &mut RenderPass<'a>, polygon_mode: PolygonMode) {
        match polygon_mode {
            PolygonMode::Fill => render_pass.draw(VERTEX_RANGE, 0..1),
            PolygonMode::Line => {
                render_pass.set_index_buffer(self.index_buffer.slice(..), IndexFormat::Uint16);
                render_pass.draw_indexed(INDEX_RANGE, 0, 0..1);
            }
            PolygonMode::Point => unimplemented!(),
        }
    }

    pub fn draw<'a>(
        &'a self,
        device: &Device,
        render_pass: &mut RenderPass<'a>,
        rect: &Rect,
        color: &Color,
        polygon_mode: PolygonMode,
    ) {
        render_pass.set_viewport(rect.x(), rect.y(), rect.width(), rect.height(), 0.0, 1.0);
        render_pass.set_pipeline(self.pipeline(polygon_mode));

        let bind = BINDS
            .get_mut()
            .entry(*color)
            .or_insert_with(|| Self::bind_group_with_color(&self.bind_group_layout, device, color));

        render_pass.set_bind_group(0, bind, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        self.draw_vertices(render_pass, polygon_mode);
    }
}
