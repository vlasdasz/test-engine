use rtools::Rglica;
use sprites::Sprite;
use ui::{view_base::ViewBase, View};

#[derive(Default)]
pub struct LevelView {
    base:   ViewBase,
    sprite: Rglica<dyn Sprite>,
}

impl LevelView {
    pub fn set_sprite(&mut self, sprite: Rglica<dyn Sprite>) {
        self.sprite = sprite
    }
}

impl View for LevelView {
    fn view(&self) -> &ViewBase {
        &self.base
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }
}
