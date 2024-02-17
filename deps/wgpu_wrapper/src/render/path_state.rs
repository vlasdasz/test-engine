use std::ops::Range;

use gm::flat::{Point, Rect};
use wgpu::{
    include_wgsl, BindGroup, BindGroupLayout, BindGroupLayoutEntry, BindingType, Buffer, BufferBindingType,
    Device, PipelineLayoutDescriptor, PolygonMode, RenderPass, RenderPipeline, ShaderStages, TextureFormat,
};

use crate::utils::make_pipeline;

#[derive(Debug)]
pub struct PathState {
    pub(crate) bind_group_layout: BindGroupLayout,
    fill_pipeline:                RenderPipeline,
    line_pipeline:                RenderPipeline,
}

impl PathState {
    pub fn new(device: &Device, texture_format: TextureFormat) -> Self {
        let shader = device.create_shader_module(include_wgsl!("shaders/path.wgsl"));

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label:                Some("Path Pipeline Layout"),
            bind_group_layouts:   &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let fill_pipeline = make_pipeline::<Point>(
            "Path Fill Render Pipeline",
            device,
            &pipeline_layout,
            &shader,
            texture_format,
            PolygonMode::Fill,
        );

        let line_pipeline = make_pipeline::<Point>(
            "Path Line Render Pipeline",
            device,
            &pipeline_layout,
            &shader,
            texture_format,
            PolygonMode::Line,
        );

        Self {
            bind_group_layout,
            fill_pipeline,
            line_pipeline,
        }
    }

    fn pipeline(&self, polygon_mode: PolygonMode) -> &RenderPipeline {
        match polygon_mode {
            PolygonMode::Fill => &self.fill_pipeline,
            PolygonMode::Line => &self.line_pipeline,
            PolygonMode::Point => unimplemented!(),
        }
    }

    pub fn draw_buffer<'a>(
        &'a self,
        render_pass: &mut RenderPass<'a>,
        rect: &Rect,
        polygon_mode: PolygonMode,
        buffer: &'a Buffer,
        bind_group: &'a BindGroup,
        vertex_range: Range<u32>,
    ) {
        render_pass.set_viewport(rect.x(), rect.y(), rect.width(), rect.height(), 0.0, 1.0);
        render_pass.set_pipeline(self.pipeline(polygon_mode));

        render_pass.set_bind_group(0, bind_group, &[]);
        render_pass.set_vertex_buffer(0, buffer.slice(..));
        render_pass.draw(vertex_range, 0..1);
    }
}
