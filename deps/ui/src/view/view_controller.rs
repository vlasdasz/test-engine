use refs::{Own, ToWeak};
use rtools::Animation;

use crate::{UIAnimation, UIManager, View, ViewAnimation, ViewFrame, ViewSubviews};
pub trait ViewController {
    fn push(&mut self, view: Own<dyn View>);
    fn pop(self: Weak<Self>);
    fn present(&mut self, view: Own<dyn View>);
}

use refs::Weak;

impl<T: ?Sized + View + 'static> ViewController for T {
    fn push(&mut self, view: Own<dyn View>) {
        if UIManager::touch_disabled() {
            return;
        }

        UIManager::disable_touch();

        let mut view = self.add_subview(view);
        view.place.as_background();
        view.set_frame(self.frame().with_zero_origin());

        UIManager::get().touch_stack.push(view.weak_view());

        let anim = UIAnimation::new(view, Animation::new(self.width(), 0, 0.5), |view, x| {
            view.set_x(x);
        });

        anim.on_finish.sub(|_| {
            UIManager::enable_touch();
        });

        self.add_animation(anim);
    }

    fn pop(mut self: Weak<Self>) {
        if UIManager::touch_disabled() {
            return;
        }

        UIManager::disable_touch();

        let anim = UIAnimation::new(self, Animation::new(0, self.width(), 0.5), |view, x| {
            view.set_x(x);
        });

        anim.on_finish.sub(move |_| {
            self.is_hidden = true;
            self.remove_from_superview();
            let vo = UIManager::get().touch_stack.pop().expect("add error message");
            assert_eq!(self.addr(), vo.addr());
            UIManager::enable_touch();
        });

        self.add_animation(anim);
    }

    fn present(&mut self, view: Own<dyn View>) {
        if UIManager::touch_disabled() {
            return;
        }

        UIManager::disable_touch();

        let mut view = UIManager::root_view().add_subview(view);
        view.place.as_background();
        view.set_frame(self.frame().with_zero_origin());
        let anim = UIAnimation::new(view, Animation::new(self.height(), 0, 0.5), |view, y| {
            view.set_y(y);
        });

        let mut this = self.weak();
        anim.on_finish.sub(move |_| {
            this.remove_from_superview();
            UIManager::enable_touch();
        });

        self.add_animation(anim);
    }
}
