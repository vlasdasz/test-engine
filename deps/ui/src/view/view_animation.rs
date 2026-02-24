use std::ops::DerefMut;

use educe::Educe;
use gm::Animation;
use refs::Weak;
use vents::OnceEvent;

// use vents::OnceEvent;
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
}

impl UIAnimation {
    pub fn new(animation: Animation, action: impl FnMut(&mut dyn View, f32) + Send + 'static) -> Self {
        Self {
            view: Weak::default(),
            animation,
            action: Box::new(action),
            on_finish: OnceEvent::default(),
        }
    }

    pub(crate) fn active(&self) -> bool {
        !self.animation.finished() && self.view.is_ok()
    }

    pub(crate) fn commit(&mut self) {
        (self.action)(self.view.deref_mut(), self.animation.value());
    }
}
