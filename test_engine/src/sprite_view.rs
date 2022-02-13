use rtools::Rglica;
use sprites::Sprite;
use ui::{
    complex::LabeledView,
    view_base::{init_view_on, ViewBase},
    View,
};

#[derive(Default)]
pub struct SpriteView {
    base:     ViewBase,
    position: Rglica<LabeledView>,
    size:     Rglica<LabeledView>,
    color:    Rglica<LabeledView>,
    sprite:   Rglica<dyn Sprite>,
}

impl SpriteView {
    pub fn set_sprite(&mut self, sprite: Rglica<dyn Sprite>) {
        self.sprite = sprite.clone();
        if sprite.is_null() {
            self.position.clear();
            self.size.clear();
            self.color.clear();
            return;
        }
        self.position.set_value(sprite.position());
        self.size.set_value(sprite.size());
        self.color.set_value(sprite.color());
    }
}

impl View for SpriteView {
    fn setup(&mut self) {
        self.position = init_view_on(self);
        self.size = init_view_on(self);
        self.color = init_view_on(self);

        self.position.set_label("position:");
        self.size.set_label("size:");
        self.color.set_label("color:");
    }

    fn layout(&mut self) {
        self.place().subviews_vertically()
    }

    fn update(&mut self) {
        self.set_sprite(self.sprite.clone());
    }

    fn view(&self) -> &ViewBase {
        &self.base
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }
}
