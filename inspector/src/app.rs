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
        Box::new(Self)
    }

    fn make_root_view(&self) -> Own<dyn View> {
        MainScreen::new()
    }

    fn initial_size(&self) -> Size {
        (1200, 1200).into()
    }

    fn enable_inspection(&self) -> bool {
        false
    }

    fn after_launch(&self) {
        // test_engine::ui::UIManager::enable_debug_frames();
    }
}
