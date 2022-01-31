use std::rc::Rc;

use sprites::{Level, SpritesDrawer};
use ui::View;

pub trait GameView: View {
    fn level(&self) -> &dyn Level;
    fn level_mut(&mut self) -> &mut dyn Level;
    fn set_drawer(&mut self, drawer: Rc<dyn SpritesDrawer>) {
        self.level_mut().set_drawer(drawer)
    }
    fn drawer(&self) -> &dyn SpritesDrawer {
        self.level().drawer()
    }
}
