use debug::LevelCommand;
use test_engine::{
    refs::Weak,
    ui::{HasText, Label, Setup, Slider, ViewData, view},
};
use tokio::spawn;

use crate::interface::main_view::client;

#[view]
pub struct LevelView {
    #[init]
    pub label: Label,
    pub scale: Slider,
}

impl Setup for LevelView {
    fn setup(mut self: Weak<Self>) {
        self.label.set_text("Scale");
        self.label.place().size(100, 50).l(20).center_y();

        self.scale.place().trb(20).w(100);
        self.scale.set_min(0.1);
        self.scale.set_max(100);
        self.scale.on_change.val(move |val| {
            self.label.set_text(val);
            spawn(async move {
                client().await.send(LevelCommand::SetScale(val)).await.unwrap();
            });
        });
    }
}
