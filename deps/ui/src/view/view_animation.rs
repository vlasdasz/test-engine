use std::ops::DerefMut;

use educe::Educe;
use gm::Animation;
use netrun::Function;
use refs::Weak;
use vents::OnceEvent;

use crate::{View, WeakView};

type Action = Box<dyn FnMut(&mut dyn View, f32) + Send>;

#[derive(Educe)]
#[educe(Debug)]
pub struct UIAnimation {
    #[educe(Debug(ignore))]
    pub(crate) view: WeakView,
    animation:       Animation,
    #[educe(Debug(ignore))]
    action:          Action,
    #[educe(Debug(ignore))]
    pub on_finish:   OnceEvent,

    finish_condition: Function<(), bool>,
}

impl UIAnimation {
    pub fn new(action: impl FnMut(&mut dyn View, f32) + Send + 'static) -> Self {
        Self {
            view:             Weak::default(),
            animation:        Animation::default(),
            action:           Box::new(action),
            on_finish:        OnceEvent::default(),
            finish_condition: Function::default(),
        }
    }

    pub fn animation(mut self, animation: Animation) -> Self {
        self.animation = animation;
        self
    }

    pub fn finish_condition(self, mut finish: impl FnMut() -> bool + Send + 'static) -> Self {
        self.finish_condition.replace(move |()| finish());
        self
    }

    pub(crate) fn active(&self) -> bool {
        if self.view.is_null() {
            return false;
        }

        if self.finish_condition.is_empty() {
            self.animation.active()
        } else {
            !self.finish_condition.call(())
        }
    }

    pub(crate) fn commit(&mut self) {
        (self.action)(self.view.deref_mut(), self.animation.value());
    }
}
