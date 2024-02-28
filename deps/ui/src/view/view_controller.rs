use refs::Own;
use rtools::Animation;

use crate::{NavigationView, UIAnimation, UIManager, View, ViewAnimation, ViewData, ViewFrame, ViewSubviews};

pub trait ViewController {
    fn navigation(&self) -> Weak<NavigationView>;
    fn present(self: Weak<Self>, view: Own<dyn View>);
}

use dispatch::on_main;
use gm::Color;
use refs::Weak;

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

    fn present(mut self: Weak<Self>, mut view: Own<dyn View>) {
        UIManager::disable_touch();

        on_main(move || {
            view.set_color(Color::WHITE);
            let mut view = UIManager::root_view().add_subview(view);
            view.set_frame(self.frame().with_zero_origin());
            let anim = UIAnimation::new(Animation::new(self.height(), 0.0, 0.4), |view, y| {
                view.set_y(y);
            });

            anim.on_finish.sub(move || {
                self.remove_from_superview();
                view.place().back();
                UIManager::enable_touch();
            });

            view.add_animation(anim);
        });
    }
}
