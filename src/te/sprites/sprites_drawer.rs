use crate::gm::{Point, Size};
use crate::sprites::Sprite;
use crate::te::Assets;
use std::rc::Rc;

pub struct SpritesDrawer {
    assets: Rc<Assets>,
}

impl SpritesDrawer {
    pub fn new(assets: Rc<Assets>) -> Self {
        Self { assets }
    }

    pub fn set_size(&self, size: &Size) {
        self.assets.shaders.sprite.enable();
        self.assets.shaders.sprite.set_resolution(size)
    }

    pub fn set_camera_position(&self, pos: &Point) {
        self.assets.shaders.sprite.enable();
        self.assets.shaders.sprite.set_camera_position(pos);
    }

    pub fn draw(&self, sprite: &Sprite) {
        let shader = &self.assets.shaders.sprite;

        shader.enable();

        shader.set_size(&sprite.size);
        shader.set_position(&sprite.position);
        shader.set_rotation(sprite.rotation);
        shader.set_color(&sprite.color);

        self.assets.buffers.fullscreen.draw();
    }
}
