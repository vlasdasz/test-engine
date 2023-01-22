use test_engine::{ui::layout::Anchor, view};
use ui::{
    refs::{Own, Weak},
    BaseView, SubView, UIManager, ViewFrame, ViewSetup,
};
use ui_views::Button;

use crate::test_game::TestGameView;

#[view]
#[derive(Default)]
pub struct UITestView {
    container: SubView<BaseView>,
    top_view:  SubView<BaseView>,
    test:      SubView<BaseView>,
    back:      SubView<Button>,
}

impl ViewSetup for UITestView {
    fn setup(mut self: Weak<Self>) {
        self.container.set_frame((200, 200, 280, 280));

        self.top_view.place.lrt(10).h(50);

        let this = self;

        self.test.place.lrb(10).anchor(this.top_view, Anchor::Top, 20);

        self.back.set_text("Back").place.size(120, 20).b(20).center_hor();

        self.back.on_tap.sub(|_| {
            UIManager::set_view(Own::<TestGameView>::default());
        });
    }
}
