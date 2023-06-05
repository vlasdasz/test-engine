use dispatch::on_main;
use refs::{Own, ToOwn, Weak};
use rtools::Animation;
use ui_proc::view;

use crate as ui;
use crate::{
    view::{ViewAnimation, ViewFrame, ViewSubviews},
    TouchStack, UIAnimation, UIManager, View, ViewSetup,
};

#[view]
pub struct NavigationView {
    first_view: Option<Own<dyn View>>,
}

impl NavigationView {
    pub fn with_view(first_view: impl View + 'static) -> Own<Self> {
        Self {
            first_view: Some(first_view.to_own()),
            ..Default::default()
        }
        .to_own()
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
    pub fn push(mut self: Weak<Self>, view: impl View + 'static) {
        assert!(!self.subviews.is_empty(), "BUG: push from empty navigation");

        let view = view.to_own();

        on_main(move || {
            UIManager::disable_touch();
            TouchStack::push_layer(view.weak_view());

            let mut prev_view = self.subviews.first().unwrap().weak_view();

            let mut view = self.add_subview(view);
            view.place.as_background();
            view.navigation_view = self;
            view.set_frame(self.frame().with_zero_origin());

            let anim = UIAnimation::new(Animation::new(self.width(), 0, 0.5), |view, x| {
                view.set_x(x);
            });

            anim.on_finish.sub(move || {
                UIManager::enable_touch();
                prev_view.is_hidden = true;
            });

            view.add_animation(anim);
        });
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
            to_pop.remove_from_superview();
            TouchStack::pop_layer(to_pop);
            UIManager::enable_touch();
        });

        to_pop.add_animation(anim);
    }

    fn below_pop(&self) -> Weak<dyn View> {
        self.subviews[self.subviews.len() - 2].weak_view()
    }
}
