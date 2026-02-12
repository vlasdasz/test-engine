use ::test_engine::{
    refs::Weak,
    ui::{Label, NumberView, Setup, view},
};
use test_engine::ui::{TextAlignment, UIManager, ViewData};

#[view]
pub struct ScaleCell {
    #[init]
    label:  Label,
    number: NumberView,
}

impl Setup for ScaleCell {
    fn setup(self: Weak<Self>) {
        self.place().distribute_ratio([4, 1]);

        let scale = UIManager::scale();

        self.label
            .set_alignment(TextAlignment::Left)
            .set_text(format!("UI scale: {scale}"));
        self.number.set_min(0.2).set_step(0.2).set_value(scale);
        self.number.on_change(move |scale| {
            UIManager::set_scale(scale);
            self.label.set_text(format!("UI scale: {scale:.2}"));
        });
    }
}
