use gl_image::Image;
use gm::Color;
use rtools::{data_manager::Handle, Animation, Rglica, ToRglica, Unwrap};

use crate::{
    basic::Button,
    complex::{DrawingView, TableView},
    impl_view,
    test::{layout_view::LayoutView, subviews_test_view::SubviewsTestView},
    view,
    view::{ViewData, ViewFrame, ViewSubviews},
    ImageView, Label, View, ViewBase, ViewCallbacks,
};

#[view]
#[derive(Default, Debug)]
pub struct TestView {
    label:    Rglica<Label>,
    button:   Rglica<Button>,
    image:    Rglica<ImageView>,
    subviews: Rglica<SubviewsTestView>,
    drawing:  Rglica<DrawingView>,
    layout:   Rglica<LayoutView>,
    animated: Rglica<ImageView>,
    table:    Rglica<TableView<String>>,

    animation: Unwrap<Animation>,

    label_value: u64,
}

impl_view!(TestView);

impl TestView {
    pub fn set_image(&mut self, image: Handle<Image>) -> &mut Self {
        self.image.set_image(image);
        self
    }

    pub fn set_button_image(&mut self, image: Handle<Image>) -> &mut Self {
        self.button.set_image(image);
        self
    }

    pub fn set_animation_image(&mut self, image: Handle<Image>) -> &mut Self {
        self.animated.set_image(image);
        self
    }
}

impl ViewCallbacks for TestView {
    fn setup(&mut self) {
        self.label = self.add_view();
        self.label.set_text("Hello label!");

        self.button = self.add_view();
        self.button.on_tap.set(self, |this, _| {
            let val = this.label_value;
            this.label.set_text(format!("Hello label! {}", val));
            this.label_value += 1;
        });

        self.image = self.add_view();

        self.subviews = self.add_view();

        self.drawing = self.add_view();
        self.drawing.add_path(
            vec![(20, 20), (30, 20), (20, 40), (30, 50), (1, 60), (1, 20)],
            Color::GREEN,
        );

        self.layout = self.add_view();

        self.animated = self.add_view();
        self.animated.set_frame((100, 100));

        self.animation = Animation::new(0, 400, 10).into();

        self.table = self.add_view();
        self.table
            .set_data(vec!["spika".into(), "rglica".into(), "sokol".into()]);
    }

    fn layout(&mut self) {
        self.place().all_vertically();
        self.animated.set_y(self.animation.value());
    }
}
