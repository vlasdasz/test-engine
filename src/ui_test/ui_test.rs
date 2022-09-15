use test_engine::{
    rtools::{Rglica, ToRglica},
    ui::{
        basic::Button, view, BaseView, SubView, View, ViewBase, ViewCallbacks, ViewFrame, ViewLayout,
        ViewSubviews,
    },
    ui_layer::UILayer,
};

use crate::test_game::TestGameView;

#[view]
#[derive(Default)]
pub struct UITestView {
    container: SubView<BaseView>,
    top_view:  SubView<BaseView>,
    test:      SubView<BaseView>,
    back:      SubView<Button>,
    ui:        Rglica<UILayer>,
}

impl ViewCallbacks for UITestView {
    fn setup(&mut self) {
        self.container.set_frame((200, 200, 280, 280));

        self.top_view.make_layout(|a| {
            a.left().top().right().val(10);
            a.height(50);
        });

        self.test.make_layout(|a| {
            a.top().anchor(self.top_view, 20);
            a.left().right().bottom().val(10);
        });

        self.back.set_text("Back").make_layout(|l| {
            l.width(120).height(20);
            l.bottom().val(20);
            l.center_hor();
        });
        self.back.on_tap.set(self, |this, _| {
            this.ui.set_view::<TestGameView>();
        });
    }
}
