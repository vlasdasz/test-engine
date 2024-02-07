use ui::{refs::Weak, view, SubView, ViewData, ViewSetup, ViewSubviews};
use ui_views::{touch_test_view::TouchTestView, ScrollView};

#[view]
struct ScrollViewTest {
    scroll: SubView<ScrollView>,
}

impl ViewSetup for ScrollViewTest {
    fn setup(mut self: Weak<Self>) {
        self.scroll.place().back();
        self.scroll.content_size = (1000, 1500).into();

        let touch1 = self.scroll.add_view::<TouchTestView>();
        touch1.place().clear().size(200, 200).center();

        let touch2 = self.scroll.add_view::<TouchTestView>();
        touch2.place().clear().size(200, 200).bl(10);
    }
}

#[ignore]
#[test]
fn test() {
    old_engine::ViewApp::<ScrollViewTest>::start().unwrap()
}
