#![allow(dead_code)]

use std::ops::Range;

use bytemuck::Pod;
use gm::{
    checked_usize_to_u32,
    flat::{Point, Vertex2D},
};
use indexmap::IndexMap;
use refs::Weak;
use wgpu::{
    Buffer, PipelineLayoutDescriptor, PrimitiveTopology, RenderPass, RenderPipeline, ShaderModuleDescriptor,
    ShaderSource, ShaderStages,
};
use window::{BufferUsages, DeviceHelper, PolygonMode, Window, image::Image};

use crate::{
    uniform::{UniformBind, make_uniform_layout},
    vec_buffer::VecBuffer,
    vertex_layout::VertexLayout,
};

const VERTICES: &[Point] = &[
    Point::new(-1.0, 1.0),
    Point::new(-1.0, -1.0),
    Point::new(1.0, 1.0),
    Point::new(1.0, -1.0),
];

pub(super) const VERTEX_RANGE: Range<u32> = 0..checked_usize_to_u32(VERTICES.len());

const TEXTURED_VERTICES: &[Vertex2D; 4] = &[
    Vertex2D {
        pos: Point::new(-1.0, 1.0),
        uv:  Point::new(0.0, 0.0),
    },
    Vertex2D {
        pos: Point::new(-1.0, -1.0),
        uv:  Point::new(0.0, 1.0),
    },
    Vertex2D {
        pos: Point::new(1.0, 1.0),
        uv:  Point::new(1.0, 0.0),
    },
    Vertex2D {
        pos: Point::new(1.0, -1.0),
        uv:  Point::new(1.0, 1.0),
    },
];

const TEXTURED_VERTEX_RANGE: Range<u32> = 0..checked_usize_to_u32(VERTICES.len());

pub struct RectPipeline<
    const WITH_IMAGE: bool,
    const SHADER: &'static str,
    const SHADER_CODE: &'static str,
    View,
    Instance,
> {
    pipeline: RenderPipeline,

    vertex_buffer: Buffer,

    view: UniformBind<View>,

    instances: IndexMap<Weak<Image>, VecBuffer<Instance>>,
}

impl<
    const WITH_IMAGE: bool,
    const NAME: &'static str,
    const SHADER_CODE: &'static str,
    View: Default + Pod,
    Instance: VertexLayout,
> Default for RectPipeline<WITH_IMAGE, NAME, SHADER_CODE, View, Instance>
{
    fn default() -> Self {
        let device = Window::device();

        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label:  Some(&format!("{NAME}.wgsl")),
            source: ShaderSource::Wgsl(SHADER_CODE.into()),
        });

        let sprite_view_layout =
            make_uniform_layout(&format!("{NAME}_uniform_layout"), ShaderStages::VERTEX_FRAGMENT);

        let mut bind_group_layouts = vec![&sprite_view_layout];

        let image_layout = Image::uniform_layout();

        if WITH_IMAGE {
            bind_group_layouts.push(&image_layout);
        }

        let uniform_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label:                Some(&format!("{NAME}_pipeline_layout")),
            bind_group_layouts:   &bind_group_layouts,
            push_constant_ranges: &[],
        });

        let pipeline = if WITH_IMAGE {
            device.pipeline(
                &format!("{NAME}_pipeline"),
                &uniform_layout,
                &shader,
                PolygonMode::Fill,
                PrimitiveTopology::TriangleStrip,
                &[Vertex2D::VERTEX_LAYOUT, Instance::VERTEX_LAYOUT],
            )
        } else {
            device.pipeline(
                &format!("{NAME}_pipeline"),
                &uniform_layout,
                &shader,
                PolygonMode::Fill,
                PrimitiveTopology::TriangleStrip,
                &[Point::VERTEX_LAYOUT, Instance::VERTEX_LAYOUT],
            )
        };

        let vertex_buffer = if WITH_IMAGE {
            device.buffer(TEXTURED_VERTICES, BufferUsages::VERTEX)
        } else {
            device.buffer(VERTICES, BufferUsages::VERTEX)
        };

        Self {
            pipeline,
            vertex_buffer,
            view: sprite_view_layout.into(),
            instances: IndexMap::default(),
        }
    }
}

impl<
    const WITH_IMAGE: bool,
    const SHADER: &'static str,
    const SHADER_CODE: &'static str,
    View: Pod + PartialEq,
    Instance: Pod,
> RectPipeline<WITH_IMAGE, SHADER, SHADER_CODE, View, Instance>
{
    pub fn add(&mut self, instance: Instance) {
        self.instances.entry(Weak::default()).or_default().push(instance);
    }

    pub fn add_with_image(&mut self, instance: Instance, image: Weak<Image>) {
        self.instances.entry(image).or_default().push(instance);
    }

    pub fn draw<'a>(&'a mut self, render_pass: &mut RenderPass<'a>, view: View) {
        if self.instances.is_empty() {
            return;
        }

        render_pass.set_pipeline(&self.pipeline);

        self.view.update(view);

        for (image, instances) in &mut self.instances {
            if instances.is_empty() {
                continue;
            }

            instances.load();

            render_pass.set_bind_group(0, self.view.bind(), &[]);

            if WITH_IMAGE {
                render_pass.set_bind_group(1, &image.bind, &[]);
            }

            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, instances.buffer().slice(..));

            render_pass.draw(
                if WITH_IMAGE {
                    TEXTURED_VERTEX_RANGE
                } else {
                    VERTEX_RANGE
                },
                0..instances.len(),
            );
        }
    }
}
