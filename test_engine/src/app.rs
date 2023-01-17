use std::path::PathBuf;

use gm::flat::Size;
use ui::{refs::Own, View};

use crate::AppCore;

pub trait App {
    fn setup()
    where
        Self: Sized,
    {
    }
    fn screen_size() -> Size
    where
        Self: Sized;
    fn assets_path() -> PathBuf
    where
        Self: Sized;
    fn make_root_view() -> Own<dyn View>
    where
        Self: Sized;
    fn core(&mut self) -> &mut AppCore;

    #[cfg(desktop)]
    fn make_core() -> AppCore
    where
        Self: Sized,
    {
        Self::setup();
        AppCore::new(Self::screen_size(), Self::assets_path(), Self::make_root_view())
    }

    #[cfg(desktop)]
    fn launch(&mut self) {
        self.core().screen.start_main_loop();
    }
}
