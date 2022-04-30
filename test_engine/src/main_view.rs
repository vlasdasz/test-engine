use rtools::Rglica;
use sprites::{Level, Player, SpritesDrawer};
use ui::{Touch, View};

use crate::ui_layer::UILayer;

pub trait MainView: View + HasLevel {
    fn set_ui(&mut self, _: Rglica<UILayer>) {}
}

pub trait HasLevel {
    fn player(&self) -> Rglica<Player> {
        Default::default()
    }

    fn level(&self) -> Rglica<dyn Level> {
        Default::default()
    }

    fn set_sprites_drawer(&mut self, drawer: Rglica<dyn SpritesDrawer>) {
        if self.level().is_ok() {
            self.level().set_drawer(drawer)
        }
    }

    fn pass_touch_to_level(&mut self, touch: Touch) {
        if self.level().is_null() {
            return;
        }
        self.level().set_cursor_position(touch.position);
        if touch.is_began() {
            self.level().add_touch(touch.position)
        }
    }
}
