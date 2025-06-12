use debug::UICommand;
use test_engine::{
    refs::Weak,
    ui::{HasText, Label, Setup, Slider, ViewData, view},
};
use tokio::spawn;

use crate::interface::main_view::client;

#[view]
pub struct UIView {
    #[init]
    pub title: Label,
    pub label: Label,
    pub scale: Slider,
}

impl Setup for UIView {
    fn setup(mut self: Weak<Self>) {
        self.title.place().tl(5).size(200, 50);
        self.title.set_text("UI Scale");

        self.label.set_text("Scale");
        self.label.place().size(100, 50).l(20).center_y();

        self.scale.place().trb(20).w(100);
        self.scale.set_min(0.1);
        self.scale.set_max(4);
        self.scale.on_change.val(move |val| {
            self.label.set_text(val);
            spawn(async move {
                client().await.send(UICommand::SetScale(val)).await.unwrap();
            });
        });
    }
}
