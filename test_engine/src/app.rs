use std::{ffi::c_int, marker::PhantomData};

use gl_wrapper::monitor::Monitor;

use crate::{game_view::GameView, Screen};

#[derive(Default)]
pub struct App<T> {
    pub screen:  Option<Screen>,
    pub monitor: Monitor,
    _view:       PhantomData<T>,
}

impl<T: GameView + 'static> App<T> {
    pub fn create_screen(&mut self, width: c_int, height: c_int) {
        let mut screen = Screen::new((width, height).into());

        screen.ui.set_view(T::boxed());
        screen.ui.add_debug_view();

        screen.add_monitor(self.monitor.clone());

        self.screen = screen.into();
    }

    pub fn set_screen_size(&mut self, width: c_int, height: c_int) {
        self.screen.as_mut().unwrap().set_size((width, height).into());
    }
}
