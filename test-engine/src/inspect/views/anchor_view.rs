use gm::color::{RED, WHITE};
use refs::Weak;
use ui::{Anchor, Container, HasText, Label, Setup, ViewData, ViewSubviews};
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

    pub fn set_anchor(mut self: Weak<Self>, anchor: Anchor) {
        if anchor == self.anchor {
            return;
        }

        self.anchor = anchor;
        self.update_anchor();
    }
}

impl AnchorView {
    #[allow(clippy::match_same_arms)]
    fn update_anchor(mut self: Weak<Self>) {
        const RATIO: f32 = 0.1;

        self.remove_all_subviews();

        let mut view = self.add_view::<Container>();
        view.set_color(RED);

        let mut width = move || {
            view.place().lr(0).relative_height(self, RATIO).center_y();
            self.add_view::<Container>()
                .set_color(RED)
                .place()
                .l(0)
                .relative_height(self, RATIO * 3.0)
                .relative_width(self, RATIO)
                .center_y();

            self.add_view::<Container>()
                .set_color(RED)
                .place()
                .r(0)
                .relative_height(self, RATIO * 3.0)
                .relative_width(self, RATIO)
                .center_y();
        };

        let mut height = move || {
            view.place().tb(0).relative_width(self, RATIO).center_x();
            self.add_view::<Container>()
                .set_color(RED)
                .place()
                .t(0)
                .relative_width(self, RATIO * 3.0)
                .relative_height(self, RATIO)
                .center_x();

            self.add_view::<Container>()
                .set_color(RED)
                .place()
                .b(0)
                .relative_width(self, RATIO * 3.0)
                .relative_height(self, RATIO)
                .center_x();
        };

        let mut max = move || {
            self.add_view::<Label>()
                .set_text("M")
                .set_text_size(59)
                .set_corner_radius(20)
                .set_color(WHITE)
                .place()
                .center()
                .relative_size(self, 0.4);
        };

        match self.anchor {
            Anchor::Top => {
                view.place().ltr(0).relative_height(self, RATIO);
            }
            Anchor::Bot => {
                view.place().rbl(0).relative_height(self, RATIO);
            }
            Anchor::Left => {
                view.place().tlb(0).relative_width(self, RATIO);
            }
            Anchor::Right => {
                view.place().trb(0).relative_width(self, RATIO);
            }
            Anchor::Width => width(),
            Anchor::Height => height(),
            Anchor::MaxWidth => {
                width();
                max();
            }
            Anchor::MaxHeight => {
                height();
                max();
            }
            Anchor::MinWidth => {}
            Anchor::MinHeight => {}
            Anchor::Size => {
                width();
                height();
            }
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
