use std::{marker::PhantomData, ops::DerefMut};

use gm::ToF32;
use refs::Weak;
use ui_proc::view;

use crate::{
    has_data::HasText,
    view::{ViewData, ViewTransition},
    Button, ToLabel, UIManager, View, ViewSetup, ViewSubviews,
};

mod test_engine {
    pub(crate) use educe;
    pub(crate) use refs;

    pub(crate) use crate as ui;
}

#[view]
pub struct TransitionButton<From: View + 'static, To: View + 'static> {
    _p:     PhantomData<(From, To)>,
    #[init]
    button: Button,
}

impl<From: View, To: View> TransitionButton<From, To> {
    pub fn set_text(&mut self, text: impl ToLabel) -> &mut Self {
        self.button.set_text(text);
        self
    }

    pub fn set_text_size(&mut self, size: impl ToF32) -> &mut Self {
        self.button.set_text_size(size);
        self
    }
}

impl<From: View, To: View + Default> ViewSetup for TransitionButton<From, To> {
    fn setup(self: Weak<Self>) {
        self.button.place().back();
        self.button.on_tap(move || {
            let from = self.find_superview::<From>();
            let mut to = To::new();
            from.transition_to(to.deref_mut());
            UIManager::set_view(to);
        });
    }
}
