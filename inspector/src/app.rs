use inspect::PORT_RANGE;
use test_engine::{
    App,
    refs::Own,
    ui::{Setup, Size, View},
};

use crate::ui::MainScreen;

pub struct InspectorApp;

impl App for InspectorApp {
    fn new() -> Box<Self>
    where Self: Sized {
        dbg!(PORT_RANGE);
        Box::new(Self)
    }

    fn make_root_view(&self) -> Own<dyn View> {
        MainScreen::new()
    }

    fn initial_size(&self) -> Size {
        (800, 800).into()
    }
}
