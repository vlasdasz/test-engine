use test_engine::gm::flat::Size;
use ui::{refs::Weak, view, SubView, ViewSetup, ViewTest};
use ui_views::{Label, Switch};

#[view]
struct SwitchTestView {
    switch: SubView<Switch>,
    label:  SubView<Label>,
}

impl ViewSetup for SwitchTestView {
    fn setup(mut self: Weak<Self>) {
        self.place.all_hor();
        self.switch.place.size(80, 40).l(20).center_ver();
        self.switch.selected.val(move |on| {
            self.label.set_text(on);
        });
    }
}

impl ViewTest for SwitchTestView {
    fn test_size() -> Size
    where Self: Sized {
        (400, 200).into()
    }
}

#[ignore]
#[test]
fn test() {
    test_engine::ViewApp::<SwitchTestView>::start();
}
