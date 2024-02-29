use std::ops::DerefMut;

use derivative::Derivative;
use rtools::Animation;
use vents::OnceEvent;

use crate::{view::view_data::ViewData, View};

type Action = Box<dyn FnMut(&mut dyn View, f32)>;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct UIAnimation {
    animation:     Animation,
    #[derivative(Debug = "ignore")]
    action:        Action,
    #[derivative(Debug = "ignore")]
    pub on_finish: OnceEvent,
}

impl UIAnimation {
    pub fn new(animation: Animation, action: impl FnMut(&mut dyn View, f32) + 'static) -> Self {
        Self {
            animation,
            action: Box::new(action),
            on_finish: Default::default(),
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
        self.animations().push(anim)
    }

    fn commit_animations(&mut self) {
        if self.animations().is_empty() {
            return;
        }

        let mut this = self.weak_view();

        for animation in this.animations() {
            animation.commit(self.weak_view().deref_mut());
            if animation.finished() {
                animation.on_finish.trigger(())
            }
        }
        self.animations().retain(|a| !a.finished());
    }
}
