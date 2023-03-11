use gl_image::ToImage;
use gm::{flat::PointsPath, Color};
use refs::Weak;
use rtools::{Animation, Unwrap};
use ui::{view, DrawMode, SubView, ViewCallbacks, ViewData, ViewFrame, ViewSetup};

use crate::{Button, DrawingView, ImageView, Label};

#[view]
pub struct TestView {
    label:    SubView<Label>,
    button:   SubView<Button>,
    image:    SubView<ImageView>,
    drawing:  SubView<DrawingView>,
    animated: SubView<ImageView>,

    animation: Unwrap<Animation>,

    label_value: u64,
}

impl TestView {
    pub fn set_image(&mut self, image: impl ToImage) -> &mut Self {
        self.image.set_image(image);
        self
    }

    pub fn set_button_image(&mut self, image: impl ToImage) -> &mut Self {
        self.button.set_image(image);
        self
    }

    pub fn set_animation_image(&mut self, image: impl ToImage) -> &mut Self {
        self.animated.set_image(image);
        self
    }
}

impl ViewSetup for TestView {
    fn setup(mut self: Weak<Self>) {
        self.place.all_ver();

        self.label.set_text("Hello label!");

        self.button.on_tap.sub(move || {
            let val = self.label_value;
            self.label.set_text(format!("Hello label! {val}"));
            self.label_value += 1;
        });

        self.drawing.add_path(
            PointsPath::rounded_rect((0, 0, 100, 40), 15, 50),
            &Color::GREEN,
            DrawMode::Outline,
        );

        self.animated.set_frame((100, 100));

        self.animation = Animation::new(0, 200, 10).into();
    }
}

impl ViewCallbacks for TestView {
    fn update(&mut self) {
        self.animated.set_y(self.animation.value());
        let radius = self.button.frame().size.height / 2.0;
        self.button.set_corner_radius(radius);
        self.button.set_size((radius * 2.0, radius * 2.0));
    }
}
