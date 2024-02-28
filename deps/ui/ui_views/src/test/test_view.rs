use refs::Weak;
use rtools::Animation;
use ui::{view, SubView, ViewCallbacks, ViewData, ViewFrame, ViewSetup};
use wgpu_wrapper::image::{Image, ToImage};

mod test_engine {
    pub(crate) use refs;
    pub(crate) use ui;
}

use crate::{Button, ImageView, Label};

#[view]
pub struct ViewWithCat {
    label:    SubView<Label>,
    button:   SubView<Button>,
    image:    SubView<ImageView>,
    // drawing:  SubView<DrawingView>,
    animated: SubView<ImageView>,

    animation: Animation,

    label_value: u64,
}

impl ViewWithCat {
    pub fn set_image(&mut self, image: impl ToImage) -> &mut Self {
        self.image.set_image(image);
        self
    }

    pub fn set_button_image(&mut self, image: Weak<Image>) -> &mut Self {
        self.button.set_image(image);
        self
    }

    pub fn set_animation_image(&mut self, image: impl ToImage) -> &mut Self {
        self.animated.set_image(image);
        self
    }
}

impl ViewSetup for ViewWithCat {
    fn setup(mut self: Weak<Self>) {
        self.place().all_ver();

        self.label.text = "Hello label!".into();

        self.button.on_tap(move || {
            let val = self.label_value;
            self.label.text = format!("Hello label! {val}");
            self.label_value += 1;
        });

        // self.drawing.add_path(
        //     PointsPath::rounded_rect((0, 0, 100, 40), 15, 50),
        //     &Color::GREEN,
        //     DrawMode::Outline,
        // );

        self.animated.set_frame((100, 100));

        self.animation = Animation::new(0.0, 200.0, 10.0);
    }
}

impl ViewCallbacks for ViewWithCat {
    fn update(&mut self) {
        self.animated.set_y(self.animation.value());
        let radius = self.button.frame().size.height / 2.0;
        self.button.set_corner_radius(radius);
        self.button.set_size((radius * 2.0, radius * 2.0));
    }
}
