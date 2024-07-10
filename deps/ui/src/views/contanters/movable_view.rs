use refs::Weak;
use ui_proc::view;

use crate::{
    Anchor::Top, HasText, HasTitle, ImageView, Label, UIImages, View, ViewData, ViewSetup, ViewSubviews,
};

mod test_engine {
    pub(crate) use educe;
    pub(crate) use refs;

    pub(crate) use crate as ui;
}

#[view]
pub struct MovableView<T: View + Default + 'static> {
    pub target_view: Weak<T>,

    #[init]
    title_label: Label,
    corner_view: ImageView,
}

impl<T: View + Default + 'static> ViewSetup for MovableView<T> {
    fn setup(mut self: Weak<Self>) {
        self.title_label.place().lrt(0).h(40);

        self.target_view = self.add_view();
        self.target_view.place().lrb(0).anchor(Top, self.title_label, 0);

        self.corner_view.set_image(UIImages::rb_corner()).place().size(28, 28).br(0);
    }
}

impl<T: View + Default + 'static> HasTitle for MovableView<T> {
    fn title(&self) -> &str {
        self.title_label.text()
    }

    fn set_title(&mut self, title: &str) {
        self.title_label.set_text(title);
    }
}
