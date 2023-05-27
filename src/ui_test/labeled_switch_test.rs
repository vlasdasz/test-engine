use test_engine::gm::flat::Size;
use ui::{refs::Weak, view, SubView, ViewSetup, ViewTest};
use ui_views::LabeledSwitch;

#[view]
struct LabeledSwitchTestView {
    switch: SubView<LabeledSwitch>,
}

impl ViewSetup for LabeledSwitchTestView {
    fn setup(mut self: Weak<Self>) {
        self.switch.place.background();
        self.switch.set_text("Lobel");
        self.switch.selected.val(move |on| {
            self.switch.set_text(format!("Lobel: {on}"));
        });
    }
}

impl ViewTest for LabeledSwitchTestView {
    fn test_size() -> Size
    where Self: Sized {
        (600, 60).into()
    }
}

#[ignore]
#[test]
fn test() {
    test_engine::ViewApp::<LabeledSwitchTestView>::start();
}
