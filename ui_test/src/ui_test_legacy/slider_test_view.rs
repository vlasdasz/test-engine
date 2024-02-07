use ui::{refs::Weak, view, SubView, ViewData, ViewSetup};
use ui_views::{Label, Slider};

#[view]
struct SliderTestView {
    label:  SubView<Label>,
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
    old_engine::ViewApp::<SliderTestView>::start().unwrap()
}
