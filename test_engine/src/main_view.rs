use rtools::Rglica;
use sprites::{Level, Player, SpritesDrawer};
use ui::{Touch, View};

use crate::ui_layer::UILayer;

pub trait MainView: View + HasLevel {
    fn set_ui(&mut self, _: Rglica<UILayer>);
}

pub trait HasLevel {
    fn has_level(&self) -> bool {
        false
    }
    fn player(&self) -> Rglica<Player> {
        todo!()
    }
    fn level(&self) -> &dyn Level {
        todo!()
    }
    fn level_mut(&mut self) -> &mut dyn Level {
        todo!()
    }

    fn set_sprites_drawer(&mut self, drawer: Rglica<dyn SpritesDrawer>) {
        self.level_mut().set_drawer(drawer)
    }

    fn pass_touch_to_level(&mut self, touch: Touch) {
        self.level_mut().set_cursor_position(touch.position);
        if touch.is_began() {
            self.level_mut().add_touch(touch.position)
        }
    }
}
