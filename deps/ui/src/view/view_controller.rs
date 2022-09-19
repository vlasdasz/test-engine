use rtools::Animation;

use crate::{UIAnimation, View, ViewAnimation, ViewFrame, ViewSubviews};

pub trait ViewController {
    fn push(&mut self, view: Box<dyn View>);
}

impl<T: ?Sized + View> ViewController for T {
    fn push(&mut self, view: Box<dyn View>) {
        let mut view = self.add_subview(view);
        view.place.as_background();
        view.set_frame(self.frame().with_zero_origin());
        let anim = UIAnimation::new(view, Animation::new(self.width(), 0, 0.5), |view, x| {
            view.set_x(x);
        });

        self.add_animation(anim);
    }
}
