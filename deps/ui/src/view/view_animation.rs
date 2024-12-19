use std::ops::DerefMut;

use educe::Educe;
use gm::Animation;
use vents::OnceEvent;

use crate::{View, view::view_data::ViewData};

type Action = Box<dyn FnMut(&mut dyn View, f32) + Send>;

#[derive(Educe)]
#[educe(Debug)]
pub struct UIAnimation {
    animation:     Animation,
    #[educe(Debug(ignore))]
    action:        Action,
    #[educe(Debug(ignore))]
    pub on_finish: OnceEvent,
}

impl UIAnimation {
    pub fn new(animation: Animation, action: impl FnMut(&mut dyn View, f32) + Send + 'static) -> Self {
        Self {
            animation,
            action: Box::new(action),
            on_finish: OnceEvent::default(),
        }
    }

    pub(crate) fn finished(&self) -> bool {
        self.animation.finished()
    }

    pub(crate) fn commit(&mut self, view: &mut dyn View) {
        (self.action)(view, self.animation.value());
    }
}

pub trait ViewAnimation {
    fn add_animation(&mut self, anim: UIAnimation);
    fn commit_animations(&mut self);
}

impl<T: ?Sized + View> ViewAnimation for T {
    fn add_animation(&mut self, anim: UIAnimation) {
        self.animations().push(anim);
    }

    fn commit_animations(&mut self) {
        if self.animations().is_empty() {
            return;
        }

        let mut this = self.weak_view();

        for animation in this.animations() {
            animation.commit(self.weak_view().deref_mut());
            if animation.finished() {
                animation.on_finish.trigger(());
            }
        }
        self.animations().retain(|a| !a.finished());
    }
}
