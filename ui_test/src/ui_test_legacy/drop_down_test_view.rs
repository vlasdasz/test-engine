use rtools::Random;
use ui::{refs::Weak, view, SubView, ViewData, ViewSetup};
use ui_views::DropDown;

#[view]
struct DropDownTestView {
    drop_down: SubView<DropDown>,
}

impl ViewSetup for DropDownTestView {
    fn setup(mut self: Weak<Self>) {
        self.drop_down.place().size(200, 40).center();
        self.drop_down.set_values(&Vec::<String>::random_count(20));
    }
}

#[ignore]
#[test]
fn test() {
    old_engine::ViewApp::<DropDownTestView>::start().unwrap()
}
