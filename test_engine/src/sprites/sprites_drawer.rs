use std::rc::Rc;
use crate::assets::Assets;
use gm::{Size, Point};
use sprites::Sprite;

pub struct SpritesDrawer {
    assets: Rc<Assets>,
}

impl SpritesDrawer {
    pub fn new(assets: Rc<Assets>) -> Self {
        Self { assets }
    }

    pub fn set_resolution(&self, size: &Size) {
        self.assets.shaders.sprite.enable();
        self.assets.shaders.sprite.set_resolution(size);
        self.assets.shaders.textured_sprite.enable();
        self.assets.shaders.textured_sprite.set_resolution(size)
    }

    pub fn set_camera_position(&self, pos: &Point) {
        self.assets.shaders.sprite.enable();
        self.assets.shaders.sprite.set_camera_position(pos);
        self.assets.shaders.textured_sprite.enable();
        self.assets.shaders.textured_sprite.set_camera_position(pos);
    }

    pub fn draw(&self, sprite: &Sprite) {
        let mut shader = &self.assets.shaders.sprite;
        let mut buffer = &self.assets.buffers.fullscreen;

        if let Some(image) = sprite.image {
            shader = &self.assets.shaders.textured_sprite;
            buffer = &self.assets.buffers.fullscreen_image;
            image.bind();
        } else {
        }

        shader.enable();

        shader.set_size(&sprite.size);
        shader.set_position(&sprite.position);
        shader.set_rotation(sprite.rotation);
        shader.set_color(&sprite.color);

        buffer.draw();
    }
}
