use gl_image::Image;
use gm::Color;
use rtools::{Rglica, ToRglica};

use crate::{
    basic::Button,
    complex::DrawingView,
    test::subviews_test_view::SubviewsTestView,
    view_base::{init_view_on, make_view_on, ViewBase},
    ImageView, Label, View,
};

#[derive(Default, Debug)]
pub struct TestView {
    base:     ViewBase,
    label:    Rglica<Label>,
    button:   Rglica<Button>,
    image:    Rglica<ImageView>,
    subviews: Rglica<SubviewsTestView>,
    drawing:  Rglica<DrawingView>,

    label_value: u64,
}

impl TestView {
    pub fn set_image(&mut self, image: Image) {
        self.image.set_image(image)
    }

    pub fn set_button_image(&mut self, image: Image) {
        self.button.set_image(image)
    }
}

impl View for TestView {
    fn setup(&mut self) {
        self.label = init_view_on(self);
        self.label.set_text("Hello label!");

        self.button = init_view_on(self);
        let mut this = self.to_rglica();
        self.button.on_tap.subscribe(move |_| {
            let val = this.label_value;
            this.label.set_text(format!("Hello label! {}", val));
            this.label_value += 1;
        });

        self.image = init_view_on(self);

        self.subviews = init_view_on(self);

        self.drawing = make_view_on(self, |drawing: &mut DrawingView| {
            drawing.add_path(
                vec![
                    (20, 20).into(),
                    (30, 20).into(),
                    (20, 40).into(),
                    (30, 50).into(),
                    (1, 60).into(),
                    (1, 20).into(),
                ]
                .into(),
                Color::GREEN,
            );
        });
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
