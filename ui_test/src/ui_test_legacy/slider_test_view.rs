use ui::{refs::Weak, view, SubView, ViewData, ViewSetup};
use ui_views::{GLLabel, Slider};

#[view]
struct SliderTestView {
    label:  SubView<GLLabel>,
    slider: SubView<Slider>,
}

impl ViewSetup for SliderTestView {
    fn setup(mut self: Weak<Self>) {
        self.label.place().l(20).center_y().size(200, 40);
        self.label.set_text("Sokol");

        self.slider.place().r(200).center_y().size(40, 500);
        self.slider.set_range(500, 0);
        self.slider.on_change.val(move |val| {
            self.label.set_text(val);
        })
    }
}

#[ignore]
#[test]
fn test() {
    test_engine::ViewApp::<SliderTestView>::start().unwrap()
}
