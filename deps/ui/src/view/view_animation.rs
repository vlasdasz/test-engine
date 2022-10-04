use std::ops::{Deref, DerefMut};

use refs::Weak;
use rtools::{Animation, Event};

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

    fn finished(&self) -> bool {
        self.animation.finished()
    }

    fn commit(&mut self) {
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
    fn commit_animations(&mut self);
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

    fn commit_animations(&mut self) {
        if UIManager::animations().is_empty() {
            return;
        }
        for animation in &mut UIManager::get().animations {
            animation.commit();
            if animation.finished() {
                animation.on_finish.trigger(())
            }
        }
        UIManager::get().animations.retain(|a| !a.finished())
    }
}
