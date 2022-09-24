use rtools::Animation;

use crate::{get_ui_drawer, UIAnimation, View, ViewAnimation, ViewFrame, ViewSubviews};

pub trait ViewController {
    fn push(&mut self, view: Box<dyn View>);
    fn present(&mut self, view: Box<dyn View>);
}

impl<T: ?Sized + View + 'static> ViewController for T {
    fn push(&mut self, view: Box<dyn View>) {
        if *get_ui_drawer().touch_disabled() {
            return;
        }

        *get_ui_drawer().touch_disabled() = true;

        let mut view = self.add_subview(view);
        view.place.as_background();
        view.set_frame(self.frame().with_zero_origin());

        let anim = UIAnimation::new(view, Animation::new(self.width(), 0, 0.5), |view, x| {
            view.set_x(x);
        });

        anim.on_finish.sub(|_| {
            *get_ui_drawer().touch_disabled() = false;
        });

        self.add_animation(anim);
    }

    fn present(&mut self, view: Box<dyn View>) {
        if *get_ui_drawer().touch_disabled() {
            return;
        }

        *get_ui_drawer().touch_disabled() = true;

        let mut view = get_ui_drawer().root_view().add_subview(view);
        let mut this = self.rglica();
        view.place.as_background();
        view.set_frame(self.frame().with_zero_origin());
        let anim = UIAnimation::new(view, Animation::new(self.height(), 0, 0.5), |view, y| {
            view.set_y(y);
        });

        anim.on_finish.sub(move |_| {
            this.remove_from_superview();
            *get_ui_drawer().touch_disabled() = false;
        });

        self.add_animation(anim);
    }
}
