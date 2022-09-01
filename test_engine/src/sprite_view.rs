use gl_image::Image;
use rtools::{data_manager::DataManager, Rglica, ToRglica};
use sprites::Sprite;
use ui::{
    basic::Button, complex::LabeledView, impl_view, view, View, ViewBase, ViewCallbacks, ViewData,
    ViewLayout, ViewSubviews,
};

#[view]
#[derive(Default, Debug)]
pub struct SpriteView {
    position: Rglica<LabeledView>,
    size:     Rglica<LabeledView>,
    color:    Rglica<LabeledView>,

    delete_button: Rglica<Button>,

    sprite: Rglica<dyn Sprite>,
}

impl_view!(SpriteView);

impl SpriteView {
    pub fn set_sprite(&mut self, sprite: Rglica<dyn Sprite>) {
        self.sprite = sprite;
        self.delete_button.set_hidden(sprite.is_null());
        if sprite.is_null() {
            self.position.clear();
            self.size.clear();
            self.color.clear();
            return;
        }
        self.position.set_value(sprite.position());
        self.size.set_value(sprite.size());
        self.color.set_value(*sprite.color());
    }

    fn setup_delete_button(&mut self) {
        self.delete_button = self.add_view();
        self.delete_button
            .set_hidden(true)
            .set_image(Image::get("delete.png"))
            .new_placer()
            .width(20)
            .height(20)
            .top()
            .left();

        self.delete_button.on_tap.set(self, |this, _| {
            if this.sprite.is_ok() {
                this.sprite.remove();
                this.set_sprite(Rglica::default());
            }
        });
    }
}

impl ViewCallbacks for SpriteView {
    fn setup(&mut self) {
        self.tiling().ver();

        (self.position, self.size, self.color) = (self.add_view(), self.add_view(), self.add_view());

        self.position.set_label("position:");
        self.size.set_label("size:");
        self.color.set_label("color:");

        self.setup_delete_button();
    }

    fn update(&mut self) {
        if self.sprite.is_null() {
            return;
        }
        self.set_sprite(self.sprite);
    }
}
