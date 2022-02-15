use gl_image::Image;
use rtools::Rglica;

use crate::{
    basic::Button,
    test::subviews_test_view::SubviewsTestView,
    view_base::{init_view_on, ViewBase},
    ImageView, Label, View,
};

#[derive(Default, Debug)]
pub struct TestView {
    base:     ViewBase,
    label:    Rglica<Label>,
    image:    Rglica<ImageView>,
    button:   Rglica<Button>,
    subviews: Rglica<SubviewsTestView>,
}

impl TestView {
    pub fn set_image(&mut self, image: Image) {
        self.image.set_image(image)
    }
}

impl View for TestView {
    fn setup(&mut self) {
        self.label = init_view_on(self);
        self.label.set_text("Hello label!");

        self.image = init_view_on(self);

        self.button = init_view_on(self);

        self.subviews = init_view_on(self);
    }

    fn layout(&mut self) {
        self.place().all_vertically();
    }

    fn view(&self) -> &ViewBase {
        &self.base
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }
}
