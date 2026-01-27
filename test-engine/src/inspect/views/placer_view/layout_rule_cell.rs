use gm::LossyConvert;
use refs::Weak;
use ui::{LayoutRule, Setup, TextField, UIEvent, ViewData, ViewFrame};
use ui_proc::view;

use crate::{inspect::views::AnchorView, ui::Anchor::Left};

mod test_engine {
    pub(crate) use educe;
    pub(crate) use refs;

    pub(crate) use crate::ui;
}

#[view]
pub struct LayoutRuleCell {
    pub editing_ended: UIEvent<f32>,

    rule: LayoutRule,

    #[init]
    anchor: AnchorView,
    value:  TextField,
}

impl Setup for LayoutRuleCell {
    fn setup(mut self: Weak<Self>) {
        self.anchor.place().l(5).center_y().custom(move |mut view| {
            let height = self.height() * 0.8;
            view.set_size(height, height);
        });

        self.value.set_text_size(20).integer_only();
        self.value
            .place()
            .center_y()
            .anchor(Left, self.anchor, 10)
            .same_height(self.anchor)
            .w(100);
        self.value.editing_ended.val(move |val| {
            self.editing_ended.trigger(val.parse().unwrap());
        });
    }
}

impl LayoutRuleCell {
    pub fn set_rule(mut self: Weak<Self>, rule: LayoutRule) {
        if let Some(anchor) = rule.side {
            self.anchor.set_anchor(anchor);
            self.value.set_text(LossyConvert::<i64>::lossy_convert(rule.offset));
        } else {
            dbg!(&rule);
        }

        self.rule = rule;
    }
}
