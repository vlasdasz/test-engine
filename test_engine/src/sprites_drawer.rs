use std::rc::Rc;

use gm::{Point, Size};
use rtools::math::IntoF32;
use sprites::SpritesDrawer;

use crate::{assets::Assets, Sprite};

#[derive(Default)]
pub struct TESpritesDrawer {
    scale:      f32,
    resolution: Size,
    assets:     Rc<Assets>,
}

impl TESpritesDrawer {
    pub fn new(assets: Rc<Assets>) -> Box<Self> {
        let mut new = Self {
            scale: 0.0,
            resolution: (0, 0).into(),
            assets,
        };
        new.set_scale(1.0);
        Box::new(new)
    }
}

impl SpritesDrawer for TESpritesDrawer {
    fn scale(&self) -> f32 {
        self.scale
    }

    fn resolution(&self) -> Size {
        self.resolution
    }

    fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
        self.assets.shaders.sprite.enable();
        self.assets.shaders.sprite.set_scale(scale.into_f32());
        self.assets.shaders.textured_sprite.enable();
        self.assets.shaders.textured_sprite.set_scale(scale);
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

    fn set_camera_position(&self, pos: Point) {
        self.assets.shaders.sprite.enable();
        self.assets.shaders.sprite.set_camera_position(pos);
        self.assets.shaders.textured_sprite.enable();
        self.assets.shaders.textured_sprite.set_camera_position(pos);
    }

    fn draw(&self, sprite: &dyn Sprite) {
        let mut shader = &self.assets.shaders.sprite;
        let mut buffer = &self.assets.buffers.fullscreen;

        if let Some(image) = sprite.image() {
            shader = &self.assets.shaders.textured_sprite;
            buffer = &self.assets.buffers.fullscreen_image;
            image.bind();
        }

        shader.enable();

        shader.set_selected(sprite.is_selected());
        shader.set_size(sprite.size());
        shader.set_position(sprite.position());
        shader.set_rotation(sprite.rotation());
        shader.set_color(sprite.color());

        buffer.draw();
    }
}
