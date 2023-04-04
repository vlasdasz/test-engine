use refs::{Own, Weak};
use rtools::Animation;
use ui_proc::view;

use crate as ui;
use crate::{
    view::{ViewAnimation, ViewFrame, ViewSubviews},
    UIAnimation, UIManager, View, ViewSetup,
};

#[view]
pub struct NavigationView {
    first_view: Option<Own<dyn View>>,
}

impl NavigationView {
    pub fn with_view<T: View + Default + 'static>() -> Own<Self> {
        let mut new = Own::<Self>::default();
        new.first_view = Some(Own::<T>::default());
        new
    }
}

impl ViewSetup for NavigationView {
    fn setup(mut self: Weak<Self>) {
        let mut view = self.first_view.take().unwrap();
        view.navigation_view = self;
        let view = self.add_subview(view);
        view.place.as_background();
    }
}

impl NavigationView {
    pub fn push(mut self: Weak<Self>, view: Own<dyn View>) {
        assert!(!self.subviews.is_empty(), "BUG: push from empty navigation");

        UIManager::disable_touch();

        let mut prev_view = self.subviews.first().unwrap().weak_view();

        let mut view = self.add_subview(view);
        view.place.as_background();
        view.navigation_view = self;
        view.set_frame(self.frame().with_zero_origin());

        UIManager::get().touch_stack.push(view.weak_view());

        let anim = UIAnimation::new(Animation::new(self.width(), 0, 0.5), |view, x| {
            view.set_x(x);
        });

        anim.on_finish.sub(move || {
            UIManager::enable_touch();
            prev_view.is_hidden = true;
        });

        view.add_animation(anim);
    }

    pub fn pop(self: Weak<Self>) {
        assert!(self.subviews.len() > 1, "BUG: Nowhere to pop");

        UIManager::disable_touch();

        let mut below = self.below_pop();
        below.is_hidden = false;
        let mut to_pop = self.subviews.last().unwrap().weak_view();

        let anim = UIAnimation::new(Animation::new(0, self.width(), 0.5), |view, x| {
            view.set_x(x);
        });

        anim.on_finish.sub(move || {
            UIManager::get().touch_stack.pop().expect("BUG: pop without push");
            to_pop.remove_from_superview();
            UIManager::enable_touch();
        });

        to_pop.add_animation(anim);
    }

    fn below_pop(&self) -> Weak<dyn View> {
        self.subviews[self.subviews.len() - 2].weak_view()
    }
}
