use rtools::Rglica;
use sprites::Sprite;
use ui::{
    basic::Button, complex::LabeledView, placer::place_vertically, view_base::ViewBase, View, ViewTemplates,
};

#[derive(Default, Debug)]
pub struct SpriteView {
    base: ViewBase,

    position: Rglica<LabeledView>,
    size:     Rglica<LabeledView>,
    color:    Rglica<LabeledView>,

    delete_button: Rglica<Button>,

    sprite: Rglica<dyn Sprite>,
}

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
        self.color.set_value(sprite.color());
    }

    fn setup_delete_button(&mut self) {
        self.delete_button = self.add_view();
        //self.delete_button.set_image(Assets::image("delete.png"));
        self.delete_button.set_hidden(true);

        self.delete_button.set_frame((20, 20));

        self.delete_button.on_tap.set(self, |_, this| {
            if this.sprite.is_ok() {
                this.sprite.remove();
                this.set_sprite(Rglica::default());
            }
        });
    }
}

impl View for SpriteView {
    fn setup(&mut self) {
        (self.position, self.size, self.color) = (self.add_view(), self.add_view(), self.add_view());

        self.position.set_label("position:");
        self.size.set_label("size:");
        self.color.set_label("color:");

        self.setup_delete_button();
    }

    fn layout(&mut self) {
        place_vertically([self.position, self.size, self.color]);
        self.delete_button.place().top_left(0);
    }

    fn update(&mut self) {
        if self.sprite.is_null() {
            return;
        }
        self.set_sprite(self.sprite);
    }

    fn view(&self) -> &ViewBase {
        &self.base
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }
}
