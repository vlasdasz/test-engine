use level::Sprite;
use refs::Weak;
use ui::{TextField, ViewCallbacks, ViewData, ViewSetup};
use ui_proc::view;

use crate as test_engine;

#[view]
pub struct SpriteView {
    #[educe(Debug(ignore))]
    sprite: Weak<dyn Sprite>,

    #[init]
    position: TextField,
    rotation: TextField,
    z_pos:    TextField,
}

impl ViewSetup for SpriteView {
    fn setup(self: Weak<Self>) {
        self.place().all_ver();
    }
}

impl ViewCallbacks for SpriteView {
    fn update(&mut self) {
        let Some(sprite) = self.sprite.get() else {
            return;
        };

        let pos = sprite.position();

        self.position.set_text(format!("{:.2} - {:.2}", pos.x, pos.y));
        self.rotation.set_text(sprite.rotation().to_string());
        self.z_pos.set_text(sprite.z_position.to_string());
    }
}

impl SpriteView {
    pub fn set_sprite(&mut self, sprite: Weak<dyn Sprite>) {
        self.sprite = sprite;
    }
}
