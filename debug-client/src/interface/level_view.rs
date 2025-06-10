use test_engine::{
    refs::Weak,
    ui::{Label, Setup, ViewData, view},
};

#[view]
pub struct LevelView {
    #[init]
    pub label: Label,
}

impl Setup for LevelView {
    fn setup(self: Weak<Self>) {
        self.label.place().back();
    }
}
