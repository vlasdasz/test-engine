use rtools::Rglica;
use sprites::{Level, SpritesDrawer};
use ui::{Touch, View};

pub trait GameView: View {
    fn level(&self) -> &dyn Level;
    fn level_mut(&mut self) -> &mut dyn Level;
    fn set_drawer(&mut self, drawer: Rglica<dyn SpritesDrawer>) {
        self.level_mut().set_drawer(drawer)
    }
    fn drawer(&self) -> &dyn SpritesDrawer {
        self.level().drawer()
    }
    fn pass_touch_to_level(&mut self, touch: Touch) {
        self.level_mut().convert_touch(touch.position)
    }
}
