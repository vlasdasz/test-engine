use gm::color::{GRAY, RED, WHITE};
use refs::Weak;
use ui::{Anchor, Container, HasText, Label, Setup, ViewData, ViewSubviews};
use ui_proc::view;

mod test_engine {
    pub(crate) use educe;
    pub(crate) use refs;

    pub(crate) use crate::ui;
}

const BORDER_WIDTH: f32 = 2.0;

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

        let mut hor_line = move || {
            self.add_view::<Container>()
                .set_color(RED)
                .place()
                .lr(BORDER_WIDTH)
                .relative_height(self, RATIO)
                .center_y()
                .view()
        };

        let mut ver_line = move || {
            self.add_view::<Container>()
                .set_color(RED)
                .place()
                .tb(BORDER_WIDTH)
                .relative_width(self, RATIO)
                .center_x()
                .view()
        };

        let mut smol_bot = move || {
            self.add_view::<Container>()
                .set_color(RED)
                .set_corner_radius(1)
                .place()
                .b(BORDER_WIDTH)
                .relative_width(self, RATIO * 3.0)
                .relative_height(self, RATIO)
                .center_x();
        };

        let mut width = move || {
            hor_line();
            self.add_view::<Container>()
                .set_color(RED)
                .set_corner_radius(1)
                .place()
                .l(BORDER_WIDTH)
                .relative_height(self, RATIO * 3.0)
                .relative_width(self, RATIO)
                .center_y();

            self.add_view::<Container>()
                .set_color(RED)
                .set_corner_radius(1)
                .place()
                .r(BORDER_WIDTH)
                .relative_height(self, RATIO * 3.0)
                .relative_width(self, RATIO)
                .center_y();
        };

        let mut height = move || {
            ver_line();

            self.add_view::<Container>()
                .set_color(RED)
                .set_corner_radius(1)
                .place()
                .t(BORDER_WIDTH)
                .relative_width(self, RATIO * 3.0)
                .relative_height(self, RATIO)
                .center_x();

            smol_bot();
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
                hor_line();
                // view.place().ltr(0).relative_height(self, RATIO);
            }
            Anchor::Bot => {
                hor_line();
                smol_bot();
                ver_line().place().t(20);
                // view.place().rbl(0).relative_height(self, RATIO);
            }
            Anchor::Left => {
                ver_line();
                // view.place().tlb(0).relative_width(self, RATIO);
            }
            Anchor::Right => {
                ver_line();
                // view.place().trb(0).relative_width(self, RATIO);
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
        self.set_color(WHITE)
            .set_corner_radius(5)
            .set_border_color(GRAY)
            .set_border_width(BORDER_WIDTH);
    }
}
