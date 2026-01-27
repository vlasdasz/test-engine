use refs::Weak;
use ui::{LayoutRule, Setup, ViewData, ViewFrame};
use ui_proc::view;

use crate::inspect::views::AnchorView;

mod test_engine {
    pub(crate) use educe;
    pub(crate) use refs;

    pub(crate) use crate::ui;
}

#[view]
pub struct LayoutRuleCell {
    rule: LayoutRule,

    #[init]
    anchor: AnchorView,
}

impl Setup for LayoutRuleCell {
    fn setup(self: Weak<Self>) {
        self.anchor.place().l(5).center_y().custom(move |mut view| {
            let height = self.height() * 0.8;
            view.set_size(height, height);
        });
    }
}

impl LayoutRuleCell {
    pub fn set_rule(mut self: Weak<Self>, rule: LayoutRule) {
        if let Some(anchor) = rule.side {
            self.anchor.set_anchor(anchor);
        } else {
            dbg!(&rule);
        }

        self.rule = rule;
    }
}
