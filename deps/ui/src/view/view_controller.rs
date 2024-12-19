use gm::Animation;
use refs::Own;

use crate::{
    NavigationView, Touch, UIAnimation, UIManager, View, ViewAnimation, ViewData, ViewFrame, ViewSubviews,
};

pub trait ViewController {
    fn navigation(&self) -> Weak<NavigationView>;
    fn present(self: Weak<Self>, view: Own<dyn View>) -> Receiver<()>;
}

use dispatch::on_main;
use gm::Color;
use refs::Weak;
use tokio::sync::oneshot::{Receiver, channel};

pub const PRESENT_ANIMATION_DURATION: f32 = 0.4;

impl<T: ?Sized + View + 'static> ViewController for T {
    fn navigation(&self) -> Weak<NavigationView> {
        assert!(
            self.superview().is_ok(),
            "Current view is not a part of navigation stack"
        );

        if self.navigation_view().is_ok() {
            self.navigation_view()
        } else {
            self.superview().navigation()
        }
    }

    fn present(mut self: Weak<Self>, mut view: Own<dyn View>) -> Receiver<()> {
        let touch_lock = Touch::lock();

        let (se, rc) = channel();

        on_main(move || {
            view.set_color(Color::WHITE);
            let mut view = UIManager::root_view_weak().__add_subview_internal(view, true);
            view.set_frame(self.frame().with_zero_origin());
            let anim = UIAnimation::new(
                Animation::new(self.height(), 0.0, PRESENT_ANIMATION_DURATION),
                |view, y| {
                    view.set_y(y);
                },
            );

            anim.on_finish.sub(move || {
                self.remove_from_superview();
                view.place().back();
                drop(touch_lock);
                _ = se.send(());
            });

            view.add_animation(anim);
        });

        rc
    }
}
