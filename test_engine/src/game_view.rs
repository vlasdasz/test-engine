use rtools::Rglica;
use sprites::{Level, SpritesDrawer};
use ui::{Touch, View};

use crate::ui_layer::UILayer;

pub trait GameView: View {
    fn level(&self) -> &dyn Level;
    fn level_mut(&mut self) -> &mut dyn Level;
    fn set_ui(&mut self, _: Rglica<UILayer>) {}
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
