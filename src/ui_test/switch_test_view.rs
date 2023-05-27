use test_engine::gm::flat::Size;
use ui::{refs::Weak, view, SubView, ViewSetup, ViewTest};
use ui_views::{Label, Switch};

#[view]
struct SwitchTestView {
    switch: SubView<Switch>,
    label:  SubView<Label>,
}

impl ViewSetup for SwitchTestView {
    fn setup(self: Weak<Self>) {
        self.place.all_ver();
    }
}

impl ViewTest for SwitchTestView {
    fn test_size() -> Size
    where Self: Sized {
        (100, 100).into()
    }
}

#[ignore]
#[test]
fn test() {
    test_engine::ViewApp::<SwitchTestView>::start();
}
