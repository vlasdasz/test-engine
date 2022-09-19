use gl_image::Image;
use rtools::{data_manager::DataManager, Rglica};
use sprites::Sprite;
use ui::{view, SubView, ViewCallbacks, ViewData};
use ui_views::{Button, LabeledView};

#[view]
#[derive(Default)]
pub struct SpriteView {
    position: SubView<LabeledView>,
    size:     SubView<LabeledView>,
    color:    SubView<LabeledView>,

    delete_button: SubView<Button>,

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
        self.position.set_text(sprite.position());
        self.size.set_text(sprite.size());
        self.color.set_text(*sprite.color());
    }

    fn setup_delete_button(&mut self) {
        self.delete_button.place.size(20, 20).tl(0);
        self.delete_button.set_hidden(true).set_image(Image::get("delete.png"));

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
        self.place.all_ver();

        self.position.set_title("position:");
        self.size.set_title("size:");
        self.color.set_title("color:");

        self.setup_delete_button();
    }

    fn update(&mut self) {
        if self.sprite.is_null() {
            return;
        }
        self.set_sprite(self.sprite);
    }
}
