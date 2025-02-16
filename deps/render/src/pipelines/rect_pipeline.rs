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
use window::{BufferUsages, PolygonMode, Window, image::Image};

use crate::{
    device_helper::DeviceHelper,
    pipelines::pipeline_type::PipelineType,
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
    const TYPE: PipelineType,
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
    const TYPE: PipelineType,
    const NAME: &'static str,
    const SHADER_CODE: &'static str,
    View: Default + Pod,
    Instance: VertexLayout,
> Default for RectPipeline<TYPE, NAME, SHADER_CODE, View, Instance>
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

        if TYPE.image() {
            bind_group_layouts.push(&image_layout);
        }

        let uniform_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label:                Some(&format!("{NAME}_pipeline_layout")),
            bind_group_layouts:   &bind_group_layouts,
            push_constant_ranges: &[],
        });

        let pipeline = if TYPE.image() {
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

        let vertex_buffer = if TYPE.image() {
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
    const TYPE: PipelineType,
    const SHADER: &'static str,
    const SHADER_CODE: &'static str,
    View: Pod + PartialEq,
    Instance: Pod,
> RectPipeline<TYPE, SHADER, SHADER_CODE, View, Instance>
{
    pub fn add(&mut self, instance: Instance) {
        assert!(TYPE.color());
        self.instances.entry(Weak::default()).or_default().push(instance);
    }

    pub fn add_with_image(&mut self, instance: Instance, image: Weak<Image>) {
        assert!(TYPE.image());
        self.instances.entry(image).or_default().push(instance);
    }

    pub fn draw<'a>(&'a mut self, render_pass: &mut RenderPass<'a>, view: View) {
        assert!(TYPE.color() || TYPE.image());

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

            if TYPE.image() {
                render_pass.set_bind_group(1, &image.bind, &[]);
            }

            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, instances.buffer().slice(..));

            render_pass.draw(TYPE.vertex_range(), 0..instances.len());
        }
    }
}
