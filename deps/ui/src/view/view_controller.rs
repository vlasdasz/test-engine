use refs::{Own, ToWeak};
use rtools::Animation;

use crate::{NavigationView, UIAnimation, UIManager, View, ViewAnimation, ViewFrame, ViewSubviews};
pub trait ViewController {
    fn navigation(&self) -> Weak<NavigationView>;
    fn present(&mut self, view: Own<dyn View>);
}

use refs::Weak;

impl<T: ?Sized + View + 'static> ViewController for T {
    fn navigation(&self) -> Weak<NavigationView> {
        if self.superview.is_null() {
            panic!("Current view is not a part of navigation stack");
        }

        if self.navigation_view.is_ok() {
            self.navigation_view
        } else {
            self.superview.navigation()
        }
    }

    fn present(&mut self, view: Own<dyn View>) {
        UIManager::disable_touch();

        let mut view = UIManager::root_view().add_subview(view);
        view.place.as_background();
        view.set_frame(self.frame().with_zero_origin());
        let anim = UIAnimation::new(Animation::new(self.height(), 0, 0.5), |view, y| {
            view.set_y(y);
        });

        let mut this = self.weak();
        anim.on_finish.sub(move || {
            this.remove_from_superview();
            UIManager::enable_touch();
        });

        self.add_animation(anim);
    }
}
