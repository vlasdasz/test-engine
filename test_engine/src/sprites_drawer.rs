use std::rc::Rc;

use gm::flat::{Point, Size};
use rtools::math::IntoF32;
use sprites::SpritesDrawer;

use crate::{assets::Assets, Sprite};

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
        self.assets.shaders.sprite.enable().set_scale(scale.into_f32());
        self.assets.shaders.image_sprite.enable().set_scale(scale);
    }

    fn resolution(&self) -> Size {
        self.resolution
    }

    fn set_resolution(&mut self, size: Size) {
        self.resolution = size;
        self.assets.shaders.sprite.enable().set_resolution(size);
        self.assets.shaders.image_sprite.enable().set_resolution(size);
    }

    fn set_camera_rotation(&self, angle: f32) {
        let angle = angle + std::f32::consts::PI / 2.0;
        self.assets.shaders.sprite.enable().set_camera_rotation(angle);
        self.assets
            .shaders
            .image_sprite
            .enable()
            .set_camera_rotation(angle);
    }

    fn camera_position(&self) -> Point {
        self.camera_posotion
    }

    fn set_camera_position(&mut self, pos: Point) {
        self.camera_posotion = pos;
        self.assets.shaders.sprite.enable().set_camera_position(pos);
        self.assets.shaders.image_sprite.enable().set_camera_position(pos);
    }

    fn draw(&self, sprite: &dyn Sprite) {
        let (shader, buffer) = if sprite.image().is_ok() {
            (&self.assets.shaders.image_sprite, &self.assets.buffers.full_image)
        } else {
            (&self.assets.shaders.sprite, &self.assets.buffers.full)
        };

        shader
            .enable()
            .set_selected(sprite.is_selected())
            .set_size(sprite.size())
            .set_position(sprite.position())
            .set_rotation(sprite.rotation());

        if let Some(image) = sprite.image().get() {
            shader.set_flipped(image.flipped).set_flipped_y(image.flipped_y);
            image.bind();
        } else {
            shader.set_color(sprite.color());
        }

        buffer.draw();
    }
}
