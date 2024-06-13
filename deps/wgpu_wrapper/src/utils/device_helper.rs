use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BlendState, Buffer, ColorTargetState,
    ColorWrites, Device, Face, FragmentState, FrontFace, MultisampleState, PipelineCompilationOptions,
    PipelineLayout, PrimitiveState, PrimitiveTopology, RenderPipeline, RenderPipelineDescriptor,
    ShaderModule, TextureFormat, VertexBufferLayout, VertexState,
};

use crate::{
    state::TEXTURE_FORMAT,
    utils::{depth_stencil_state, ToBytes},
    BufferUsages, PolygonMode,
};

pub trait DeviceHelper {
    fn buffer<T: ToBytes + ?Sized>(&self, data: &T, usage: BufferUsages) -> Buffer;

    fn bind(&self, buffer: &Buffer, layout: &BindGroupLayout) -> BindGroup;

    fn pipeline(
        &self,
        label: &str,
        layout: &PipelineLayout,
        shader: &ShaderModule,
        polygon_mode: PolygonMode,
        topology: PrimitiveTopology,
        vertex_layout: &'static [VertexBufferLayout],
    ) -> RenderPipeline;
}

impl DeviceHelper for Device {
    fn buffer<T: ToBytes + ?Sized>(&self, data: &T, usage: BufferUsages) -> Buffer {
        self.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: data.to_bytes(),
            usage,
        })
    }

    fn bind(&self, buffer: &Buffer, layout: &BindGroupLayout) -> BindGroup {
        self.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout,
            entries: &[BindGroupEntry {
                binding:  0,
                resource: buffer.as_entire_binding(),
            }],
        })
    }

    fn pipeline(
        &self,
        label: &str,
        layout: &PipelineLayout,
        shader: &ShaderModule,
        polygon_mode: PolygonMode,
        topology: PrimitiveTopology,
        vertex_layout: &'static [VertexBufferLayout],
    ) -> RenderPipeline {
        self.create_render_pipeline(&RenderPipelineDescriptor {
            label:         label.into(),
            layout:        layout.into(),
            vertex:        VertexState {
                module:              shader,
                entry_point:         "v_main",
                compilation_options: PipelineCompilationOptions::default(),
                buffers:             vertex_layout,
            },
            fragment:      FragmentState {
                module:              shader,
                entry_point:         "f_main",
                compilation_options: PipelineCompilationOptions::default(),
                targets:             &[ColorTargetState {
                    format:     TEXTURE_FORMAT,
                    blend:      BlendState::ALPHA_BLENDING.into(),
                    write_mask: ColorWrites::ALL,
                }
                .into()],
            }
            .into(),
            primitive:     PrimitiveState {
                topology,
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
}
