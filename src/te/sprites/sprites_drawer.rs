use std::rc::Rc;
use crate::te::Assets;
use crate::sprites::Sprite;

pub struct SpritesDrawer {
    assets: Rc<Assets>
}

impl SpritesDrawer {

    pub fn new(assets: Rc<Assets>) -> Self {
        Self {
            assets
        }
    }

    pub fn draw(&self, sprite: &Sprite) {

        let shader = &self.assets.shaders.sprite;

        shader.enable();

        shader.set_size(&sprite.size);
        shader.set_position(&sprite.position);
        shader.set_rotation(sprite.rotation);

        self.assets.buffers.fullscreen.draw();
    }

}
