use gm::color::{RED, WHITE};
use refs::Weak;
use ui::{Anchor, Container, Setup, ViewData, ViewSubviews};
use ui_proc::view;

mod test_engine {
    pub(crate) use educe;
    pub(crate) use refs;

    pub(crate) use crate::ui;
}

#[view]
pub struct AnchorView {
    anchor: Anchor,
}

impl AnchorView {
    pub fn anchor(&self) -> Anchor {
        self.anchor
    }

    pub fn set_anchor(&mut self, anchor: Anchor) {
        if anchor == self.anchor {
            return;
        }

        self.anchor = anchor;
        self.update_anchor();
    }
}

impl AnchorView {
    fn update_anchor(&mut self) {
        const RATIO: f32 = 0.1;

        self.remove_all_subviews();

        let mut view = self.add_view::<Container>();
        view.set_color(RED);

        match self.anchor {
            Anchor::Top => {
                view.place().ltr(0).relative_height(self, RATIO);
            }
            Anchor::Bot => {
                view.place().rbl(0).relative_height(self, RATIO);
            }
            Anchor::Left => {}
            Anchor::Right => {}
            Anchor::Width => {}
            Anchor::Height => {}
            Anchor::MaxWidth => {}
            Anchor::MaxHeight => {}
            Anchor::MinWidth => {}
            Anchor::MinHeight => {}
            Anchor::Size => {}
            Anchor::CenterX => {}
            Anchor::CenterY => {}
            Anchor::Center => {}
            Anchor::X => {}
            Anchor::Y => {}
            Anchor::None => {}
        }
    }
}

impl Setup for AnchorView {
    fn setup(mut self: Weak<Self>) {
        self.set_color(WHITE);
    }
}
