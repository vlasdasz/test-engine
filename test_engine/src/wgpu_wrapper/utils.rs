use wgpu::{
    BlendState, ColorTargetState, ColorWrites, Device, Face, FragmentState, FrontFace, MultisampleState,
    PipelineLayout, PolygonMode, PrimitiveState, PrimitiveTopology, RenderPipelineDescriptor, ShaderModule,
    TextureFormat, VertexState,
};

use crate::wgpu_wrapper::vertex_layout::VertexLayout;

pub fn make_pipeline<Vertex: VertexLayout>(
    label: &str,
    device: &Device,
    layout: &PipelineLayout,
    shader: &ShaderModule,
    texture_format: TextureFormat,
) -> wgpu::RenderPipeline {
    device.create_render_pipeline(&RenderPipelineDescriptor {
        label:         label.into(),
        layout:        Some(&layout),
        vertex:        VertexState {
            module:      &shader,
            entry_point: "v_main",
            buffers:     &[Vertex::vertex_layout()],
        },
        fragment:      FragmentState {
            module:      &shader,
            entry_point: "f_main",
            targets:     &[Some(ColorTargetState {
                format:     texture_format,
                blend:      Some(BlendState::REPLACE),
                write_mask: ColorWrites::ALL,
            })],
        }
        .into(),
        primitive:     PrimitiveState {
            topology:           PrimitiveTopology::TriangleStrip,
            strip_index_format: None,
            front_face:         FrontFace::Ccw,
            cull_mode:          Some(Face::Back),
            polygon_mode:       PolygonMode::Fill,
            unclipped_depth:    false,
            conservative:       false,
        },
        depth_stencil: None,
        multisample:   MultisampleState {
            count:                     1,
            mask:                      !0,
            alpha_to_coverage_enabled: false,
        },
        multiview:     None,
    })
}
