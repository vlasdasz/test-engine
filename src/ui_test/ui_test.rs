use rtools::Strong;
use test_engine::{ui::layout::Anchor, view};
use ui::{BaseView, SubView, UIManager, ViewCallbacks, ViewFrame};
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

impl ViewCallbacks for UITestView {
    fn setup(&mut self) {
        self.container.set_frame((200, 200, 280, 280));

        self.top_view.place.lrt(10).h(50);

        self.test.place.lrb(10).anchor(self.top_view, Anchor::Top, 20);

        self.back.set_text("Back").place.size(120, 20).b(20).center_hor();

        self.back.on_tap.sub(|_| {
            UIManager::set_view(Strong::<TestGameView>::default());
        });
    }
}
