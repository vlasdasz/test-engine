use wgpu::{
    BlendState, ColorTargetState, ColorWrites, DepthStencilState, Device, Face, FragmentState, FrontFace,
    MultisampleState, PipelineCompilationOptions, PipelineLayout, PolygonMode, PrimitiveState,
    PrimitiveTopology, RenderPipeline, RenderPipelineDescriptor, ShaderModule, TextureFormat,
    VertexBufferLayout, VertexState,
};

use crate::image::Texture;

pub fn depth_stencil_state() -> DepthStencilState {
    DepthStencilState {
        format:              Texture::DEPTH_FORMAT,
        depth_write_enabled: true,
        depth_compare:       wgpu::CompareFunction::Less,
        stencil:             wgpu::StencilState::default(),
        bias:                wgpu::DepthBiasState::default(),
    }
}

pub(crate) trait DeviceHelper {
    fn make_pipeline(
        &self,
        label: &str,
        layout: Option<&PipelineLayout>,
        shader: &ShaderModule,
        texture_format: TextureFormat,
        polygon_mode: PolygonMode,
        topology: PrimitiveTopology,
        vertex_layout: &'static [VertexBufferLayout],
    ) -> RenderPipeline;
}

impl DeviceHelper for Device {
    fn make_pipeline(
        &self,
        label: &str,
        layout: Option<&PipelineLayout>,
        shader: &ShaderModule,
        texture_format: TextureFormat,
        polygon_mode: PolygonMode,
        topology: PrimitiveTopology,
        vertex_layout: &'static [VertexBufferLayout],
    ) -> RenderPipeline {
        self.create_render_pipeline(&RenderPipelineDescriptor {
            label: label.into(),
            layout,
            vertex: VertexState {
                module:              shader,
                entry_point:         "v_main",
                compilation_options: PipelineCompilationOptions::default(),
                buffers:             vertex_layout,
            },
            fragment: FragmentState {
                module:              shader,
                entry_point:         "f_main",
                compilation_options: PipelineCompilationOptions::default(),
                targets:             &[ColorTargetState {
                    format:     texture_format,
                    blend:      BlendState::ALPHA_BLENDING.into(),
                    write_mask: ColorWrites::ALL,
                }
                .into()],
            }
            .into(),
            primitive: PrimitiveState {
                topology,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: Face::Back.into(),
                polygon_mode,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: depth_stencil_state().into(),
            multisample: MultisampleState {
                count:                     1,
                mask:                      !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        })
    }
}
