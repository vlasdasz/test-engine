use wgpu::{
    BlendState, ColorTargetState, ColorWrites, DepthStencilState, Face, FragmentState, FrontFace,
    MultisampleState, PipelineLayout, PolygonMode, PrimitiveState, PrimitiveTopology,
    RenderPipelineDescriptor, ShaderModule, TextureFormat, VertexState,
};

use crate::{image::Texture, render::vertex_layout::VertexLayout, WGPUApp};

pub fn depth_stencil_state() -> DepthStencilState {
    DepthStencilState {
        format:              Texture::DEPTH_FORMAT,
        depth_write_enabled: true,
        depth_compare:       wgpu::CompareFunction::Less,
        stencil:             wgpu::StencilState::default(),
        bias:                wgpu::DepthBiasState::default(),
    }
}

pub fn make_pipeline<Vertex: VertexLayout>(
    label: &str,
    layout: &PipelineLayout,
    shader: &ShaderModule,
    texture_format: TextureFormat,
    polygon_mode: PolygonMode,
) -> wgpu::RenderPipeline {
    WGPUApp::device().create_render_pipeline(&RenderPipelineDescriptor {
        label:         label.into(),
        layout:        layout.into(),
        vertex:        VertexState {
            module:              shader,
            entry_point:         "v_main",
            compilation_options: Default::default(),
            buffers:             &[Vertex::vertex_layout()],
        },
        fragment:      FragmentState {
            module:              shader,
            entry_point:         "f_main",
            compilation_options: Default::default(),
            targets:             &[ColorTargetState {
                format:     texture_format,
                blend:      BlendState::ALPHA_BLENDING.into(),
                write_mask: ColorWrites::ALL,
            }
            .into()],
        }
        .into(),
        primitive:     PrimitiveState {
            topology: PrimitiveTopology::TriangleStrip,
            strip_index_format: None,
            front_face: FrontFace::Ccw,
            cull_mode: Face::Back.into(),
            polygon_mode,
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil: depth_stencil_state().into(),
        multisample:   MultisampleState {
            count:                     1,
            mask:                      !0,
            alpha_to_coverage_enabled: false,
        },
        multiview:     None,
    })
}
