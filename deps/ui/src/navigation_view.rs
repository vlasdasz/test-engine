use dispatch::on_main;
use gm::{Animation, Color};
use refs::{Own, ToOwn, Weak};
use ui_proc::view;

use crate::{Touch, WeakView};

mod test_engine {
    pub(crate) use refs;

    pub(crate) use crate as ui;
}

use crate::{
    view::{ViewAnimation, ViewFrame, ViewSubviews},
    TouchStack, UIAnimation, View, ViewData, ViewSetup,
};

#[view]
pub struct NavigationView {
    first_view: Option<Own<dyn View>>,
}

impl NavigationView {
    pub fn with_view(first_view: Own<dyn View>) -> Own<Self> {
        Self {
            first_view: Some(first_view),
            ..Default::default()
        }
        .to_own()
    }
}

impl ViewSetup for NavigationView {
    fn setup(mut self: Weak<Self>) {
        let mut view = self.first_view.take().unwrap();
        view.set_navigation_view(self);
        let view = self.add_subview(view);
        view.place().back();
    }
}

impl NavigationView {
    pub fn push(mut self: Weak<Self>, mut view: Own<dyn View>) {
        assert!(!self.subviews.is_empty(), "BUG: push from empty navigation");

        let touch_lock = Touch::lock();

        on_main(move || {
            TouchStack::push_layer(view.weak_view());

            let mut prev_view = self.subviews.last().unwrap().weak_view();

            view.set_color(Color::WHITE);
            let mut view = self.add_subview(view);
            view.place().back();
            view.set_navigation_view(self);
            view.set_frame(self.frame().with_zero_origin());

            let anim = UIAnimation::new(Animation::new(self.width(), 0.0, 0.5), |view, x| {
                view.set_x(x);
            });

            anim.on_finish.sub(move || {
                drop(touch_lock);
                prev_view.set_hidden(true);
            });

            view.add_animation(anim);
        });
    }

    pub fn pop(self: Weak<Self>) {
        assert!(self.subviews.len() > 1, "BUG: Nowhere to pop");

        let touch_lock = Touch::lock();

        let mut below = self.below_pop();
        below.set_hidden(false);
        let mut to_pop = self.subviews.last().unwrap().weak_view();

        let anim = UIAnimation::new(Animation::new(0.0, self.width(), 0.5), |view, x| {
            view.set_x(x);
        });

        anim.on_finish.sub(move || {
            to_pop.remove_from_superview();
            TouchStack::pop_layer(to_pop);
            drop(touch_lock);
        });

        to_pop.add_animation(anim);
    }

    fn below_pop(&self) -> WeakView {
        self.subviews[self.subviews.len() - 2].weak_view()
    }
}
