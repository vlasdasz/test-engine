use test_engine::{
    inspect::views::AnchorView,
    refs::Weak,
    ui::{LayoutRule, Setup, ViewData, view},
};

#[view]
pub struct LayoutRuleCell {
    rule: LayoutRule,

    #[init]
    anchor: AnchorView,
}

impl Setup for LayoutRuleCell {
    fn setup(self: Weak<Self>) {
        self.place().all_hor();
    }
}
