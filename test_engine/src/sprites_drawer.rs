use std::rc::Rc;

use gm::flat::{Point, Size};
use rtools::math::IntoF32;
use sprites::SpritesDrawer;

use crate::{assets::Assets, Sprite};

#[derive(Default)]
pub struct TESpritesDrawer {
    scale:           f32,
    resolution:      Size,
    camera_posotion: Point,
    assets:          Rc<Assets>,
}

impl TESpritesDrawer {
    pub fn new(assets: Rc<Assets>) -> Box<Self> {
        let mut new = Self {
            scale: 0.0,
            resolution: (0, 0).into(),
            assets,
            camera_posotion: Default::default(),
        };
        new.set_scale(1.0);
        Box::new(new)
    }
}

impl SpritesDrawer for TESpritesDrawer {
    fn scale(&self) -> f32 {
        self.scale
    }

    fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
        self.assets.shaders.sprite.enable();
        self.assets.shaders.sprite.set_scale(scale.into_f32());
        self.assets.shaders.textured_sprite.enable();
        self.assets.shaders.textured_sprite.set_scale(scale);
    }

    fn resolution(&self) -> Size {
        self.resolution
    }

    fn set_resolution(&mut self, size: Size) {
        self.resolution = size;
        self.assets.shaders.sprite.enable();
        self.assets.shaders.sprite.set_resolution(size);
        self.assets.shaders.textured_sprite.enable();
        self.assets.shaders.textured_sprite.set_resolution(size)
    }

    fn set_camera_rotation(&self, angle: f32) {
        let angle = angle + std::f32::consts::PI / 2.0;
        self.assets.shaders.sprite.enable();
        self.assets.shaders.sprite.set_camera_rotation(angle);
        self.assets.shaders.textured_sprite.enable();
        self.assets
            .shaders
            .textured_sprite
            .set_camera_rotation(angle);
    }

    fn camera_position(&self) -> Point {
        self.camera_posotion
    }

    fn set_camera_position(&mut self, pos: Point) {
        self.camera_posotion = pos;
        self.assets.shaders.sprite.enable();
        self.assets.shaders.sprite.set_camera_position(pos);
        self.assets.shaders.textured_sprite.enable();
        self.assets.shaders.textured_sprite.set_camera_position(pos);
    }

    fn draw(&self, sprite: &dyn Sprite) {
        let mut shader = &self.assets.shaders.sprite;
        let mut buffer = &self.assets.buffers.fullscreen;

        if sprite.image().is_some() {
            shader = &self.assets.shaders.textured_sprite;
            buffer = &self.assets.buffers.fullscreen_image;
        }

        shader.enable();

        shader.set_selected(sprite.is_selected());
        shader.set_size(sprite.size());
        shader.set_position(sprite.position());
        shader.set_rotation(sprite.rotation());

        if let Some(image) = sprite.image() {
            shader.set_flipped(image.flipped);
            shader.set_flipped_y(image.flipped_y);
            image.bind();
        } else {
            shader.set_color(sprite.color());
        }

        buffer.draw();
    }
}
