use std::marker::PhantomData;

use gm::ToF32;
use refs::Weak;
use ui_proc::view;

use crate::{view::ViewData, Button, Sub, ToLabel, UIManager, View, ViewSetup};

mod test_engine {
    pub(crate) use educe;
    pub(crate) use refs;

    pub(crate) use crate as ui;
}

#[view]
pub struct GoTo<T: View + Default + 'static> {
    button: Sub<Button>,
    _p:     PhantomData<T>,
}

impl<T: View + Default> GoTo<T> {
    pub fn set_text(&mut self, text: impl ToLabel) -> &mut Self {
        self.button.set_text(text);
        self
    }

    pub fn set_text_size(&mut self, size: impl ToF32) -> &mut Self {
        self.button.set_text_size(size);
        self
    }
}

impl<T: View + Default> ViewSetup for GoTo<T> {
    fn setup(self: Weak<Self>) {
        self.button.place().back();
        self.button.on_tap(|| {
            UIManager::set_view(T::new());
        });
    }
}
