use std::ops::{Deref, DerefMut};

use refs::Weak;
use rtools::Animation;
use vents::Event;

use crate::{UIManager, View};

type Action = Box<dyn FnMut(&mut dyn View, f32)>;

pub struct UIAnimation {
    view:          Weak<dyn View>,
    animation:     Animation,
    action:        Action,
    pub on_finish: Event,
}

impl UIAnimation {
    pub fn new(
        view: impl Deref<Target = impl View + ?Sized>,
        animation: Animation,
        action: impl FnMut(&mut dyn View, f32) + 'static,
    ) -> Self {
        Self {
            view: view.weak_view(),
            animation,
            action: Box::new(action),
            on_finish: Default::default(),
        }
    }

    pub(crate) fn finished(&self) -> bool {
        self.animation.finished()
    }

    pub(crate) fn commit(&mut self) {
        (self.action)(self.view.deref_mut(), self.animation.value());
    }
}

pub trait ViewAnimation {
    fn make_animation(
        &mut self,
        _: impl Deref<Target = impl View>,
        _: Animation,
        _: impl FnMut(&mut dyn View, f32) + 'static,
    ) -> &mut Self;
    fn add_animation(&mut self, anim: UIAnimation);
}

impl<T: ?Sized + View> ViewAnimation for T {
    fn make_animation(
        &mut self,
        view: impl Deref<Target = impl View>,
        animation: Animation,
        action: impl FnMut(&mut dyn View, f32) + 'static,
    ) -> &mut Self {
        UIManager::add_animation(UIAnimation::new(view, animation, action));
        self
    }

    fn add_animation(&mut self, anim: UIAnimation) {
        UIManager::add_animation(anim)
    }
}
