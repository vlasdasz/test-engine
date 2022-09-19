use rtools::{address::Address, Animation, ToRglica};

use crate::{get_ui_drawer, UIAnimation, View, ViewAnimation, ViewFrame, ViewSubviews};

pub trait ViewController {
    fn push(&mut self, view: Box<dyn View>);
    fn present(&mut self, view: Box<dyn View>);
}

impl<T: ?Sized + View + 'static> ViewController for T {
    fn push(&mut self, view: Box<dyn View>) {
        let mut view = self.add_subview(view);
        view.place.as_background();
        view.set_frame(self.frame().with_zero_origin());
        let anim = UIAnimation::new(view, Animation::new(self.width(), 0, 0.5), |view, x| {
            view.set_x(x);
        });

        self.add_animation(anim);
    }

    fn present(&mut self, view: Box<dyn View>) {
        let mut view = self.add_subview(view);
        let mut this = self.to_rglica();
        view.place.as_background();
        view.set_frame(self.frame().with_zero_origin());
        let anim = UIAnimation::new(view, Animation::new(self.height(), 0, 0.5), |view, y| {
            view.set_y(y);
        });

        anim.on_finish.sub(move |_| {
            let index = this.subviews().iter().position(|sub| view.address() == sub.address()).unwrap();
            view.superview = Default::default();
            let view = this.remove_subview_at(index);
            get_ui_drawer().replace_view(view);
        });

        self.add_animation(anim);
    }
}
