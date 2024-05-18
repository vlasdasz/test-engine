use std::ops::Range;

use gm::flat::{Point, Rect};
use wgpu::{
    include_wgsl, BindGroup, BindGroupLayout, BindGroupLayoutEntry, BindingType, Buffer, BufferBindingType,
    PipelineLayoutDescriptor, PolygonMode, RenderPass, RenderPipeline, ShaderStages, TextureFormat,
};

use crate::{
    render::uniform::{make_bind, make_layout},
    utils::make_pipeline,
    WGPUApp,
};

#[derive(Debug)]
pub struct PathDrawer {
    pipeline: RenderPipeline,

    z_pos_layout:                 BindGroupLayout,
    pub(crate) color_size_layout: BindGroupLayout,
}

impl PathDrawer {
    pub fn new(texture_format: TextureFormat) -> Self {
        let device = WGPUApp::device();

        let shader = device.create_shader_module(include_wgsl!("shaders/path.wgsl"));

        let color_size_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label:   Some("path_bind_group_layout"),
            entries: &[
                BindGroupLayoutEntry {
                    binding:    0,
                    visibility: ShaderStages::VERTEX,
                    ty:         BindingType::Buffer {
                        ty:                 BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size:   None,
                    },
                    count:      None,
                },
                BindGroupLayoutEntry {
                    binding:    1,
                    visibility: ShaderStages::FRAGMENT,
                    ty:         BindingType::Buffer {
                        ty:                 BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size:   None,
                    },
                    count:      None,
                },
            ],
        });

        let z_pos_layout = make_layout("path_z_pos_layput", ShaderStages::VERTEX, 1);

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label:                Some("Path Pipeline Layout"),
            bind_group_layouts:   &[&z_pos_layout, &color_size_layout],
            push_constant_ranges: &[],
        });

        let pipeline = make_pipeline::<Point>(
            "Path Fill Render Pipeline",
            &pipeline_layout,
            &shader,
            texture_format,
            PolygonMode::Fill,
        );

        Self {
            pipeline,
            z_pos_layout,
            color_size_layout,
        }
    }

    pub fn draw_buffer<'a>(
        &'a self,
        render_pass: &mut RenderPass<'a>,
        rect: &Rect,
        buffer: &'a Buffer,
        bind_group: &'a BindGroup,
        vertex_range: Range<u32>,
        z_position: f32,
    ) {
        render_pass.set_viewport(rect.x(), rect.y(), rect.width(), rect.height(), 0., 1.);
        render_pass.set_pipeline(&self.pipeline);

        render_pass.set_bind_group(0, make_bind(&z_position, &self.z_pos_layout), &[]);
        render_pass.set_bind_group(1, bind_group, &[]);
        render_pass.set_vertex_buffer(0, buffer.slice(..));
        render_pass.draw(vertex_range, 0..1);
    }
}
