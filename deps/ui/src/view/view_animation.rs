use std::ops::{Deref, DerefMut};

use rtools::{Animation, Event, Rglica};

use crate::{get_ui_drawer, View};

type Action = Box<dyn FnMut(&mut dyn View, f32)>;

pub struct UIAnimation {
    view:          Rglica<dyn View>,
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
            view: view.rglica(),
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
        get_ui_drawer().animations().push(UIAnimation::new(view, animation, action));
        self
    }

    fn add_animation(&mut self, anim: UIAnimation) {
        get_ui_drawer().animations().push(anim)
    }

    fn commit_animations(&mut self) {
        if get_ui_drawer().animations().is_empty() {
            return;
        }
        for animation in get_ui_drawer().animations() {
            animation.commit();
            if animation.finished() {
                animation.on_finish.trigger(())
            }
        }
        get_ui_drawer().animations().retain(|a| !a.finished())
    }
}
