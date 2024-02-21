use wgpu::{
    BlendState, ColorTargetState, ColorWrites, DepthStencilState, Device, Face, FragmentState, FrontFace,
    MultisampleState, PipelineLayout, PolygonMode, PrimitiveState, PrimitiveTopology,
    RenderPipelineDescriptor, ShaderModule, TextureFormat, VertexState,
};

use crate::{image::Texture, render::vertex_layout::VertexLayout};

pub fn depth_stencil_state() -> DepthStencilState {
    DepthStencilState {
        format:              Texture::DEPTH_FORMAT,
        depth_write_enabled: true,
        depth_compare:       wgpu::CompareFunction::Less, // 1.
        stencil:             wgpu::StencilState::default(), // 2.
        bias:                wgpu::DepthBiasState::default(),
    }
}

pub fn make_pipeline<Vertex: VertexLayout>(
    label: &str,
    device: &Device,
    layout: &PipelineLayout,
    shader: &ShaderModule,
    texture_format: TextureFormat,
    polygon_mode: PolygonMode,
) -> wgpu::RenderPipeline {
    device.create_render_pipeline(&RenderPipelineDescriptor {
        label:         label.into(),
        layout:        layout.into(),
        vertex:        VertexState {
            module:      shader,
            entry_point: "v_main",
            buffers:     &[Vertex::vertex_layout()],
        },
        fragment:      FragmentState {
            module:      shader,
            entry_point: "f_main",
            targets:     &[Some(ColorTargetState {
                format:     texture_format,
                blend:      Some(BlendState::ALPHA_BLENDING),
                write_mask: ColorWrites::ALL,
            })],
        }
        .into(),
        primitive:     PrimitiveState {
            topology: PrimitiveTopology::TriangleStrip,
            strip_index_format: None,
            front_face: FrontFace::Ccw,
            cull_mode: Some(Face::Back),
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
