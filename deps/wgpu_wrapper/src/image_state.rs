use anyhow::Result;
use gm::{flat::Point, volume::UIVertex};
use wgpu::util::DeviceExt;

use crate::{
    image::{Image, Texture},
    utils::make_pipeline,
};

#[derive(Debug)]
pub struct ImageState {
    pub render_pipeline: wgpu::RenderPipeline,
    pub bind_group:      wgpu::BindGroup,
    pub vertex_buffer:   wgpu::Buffer,
    pub num_vertices:    u32,
}

impl ImageState {
    pub fn new(
        device: &wgpu::Device,
        texture_format: wgpu::TextureFormat,
        queue: &wgpu::Queue,
    ) -> Result<Self> {
        let shader = device.create_shader_module(wgpu::include_wgsl!("shaders/ui_image.wgsl"));

        let bind_group_layout = Image::bind_group_layout(device);

        let bytes = include_bytes!("../../../Assets/Images/happy-tree.png");
        let texture = Texture::from_bytes(&device, &queue, bytes, "happy-tree.png")?;

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout:  &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding:  0,
                    resource: wgpu::BindingResource::TextureView(&texture.view),
                },
                wgpu::BindGroupEntry {
                    binding:  1,
                    resource: wgpu::BindingResource::Sampler(&texture.sampler),
                },
            ],
            label:   Some("diffuse_bind_group"),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label:                Some("Image Pipeline Layout"),
            bind_group_layouts:   &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = make_pipeline::<UIVertex>(
            "Image Render Pipeline",
            &device,
            &pipeline_layout,
            &shader,
            texture_format,
        );

        const VERTICES: &[UIVertex] = &[
            UIVertex {
                pos: Point::new(-1.0, 1.0),
                uv:  Point::new(0.0, 0.0),
            },
            UIVertex {
                pos: Point::new(-1.0, -1.0),
                uv:  Point::new(0.0, 1.0),
            },
            UIVertex {
                pos: Point::new(1.0, 1.0),
                uv:  Point::new(1.0, 0.0),
            },
            UIVertex {
                pos: Point::new(1.0, -1.0),
                uv:  Point::new(1.0, 1.0),
            },
        ];

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label:    Some("Image Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage:    wgpu::BufferUsages::VERTEX,
        });
        let num_vertices = VERTICES.len() as u32;

        Ok(Self {
            render_pipeline,
            bind_group,
            vertex_buffer,
            num_vertices,
        })
    }

    pub fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_viewport(10.0, 10.0, 600.0, 600.0, 0.0, 1.0);
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.draw(0..self.num_vertices, 0..1);
    }
}
