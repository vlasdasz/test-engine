mod test_engine {
    pub(crate) use educe;
    pub(crate) use refs;

    pub(crate) use crate as ui;
}
use gm::{flat::Point, Color};
use refs::Weak;
use ui_proc::view;

use crate::{Container, Label, UIEvent, ViewData, ViewFrame, ViewSetup, ViewTouch};

#[view]
pub struct PositionView {
    began_pos: Point,

    #[educe(Debug(ignore))]
    pub moved: UIEvent<Point>,

    pub additional_label: Option<String>,

    #[init]
    dot:   Container,
    label: Label,
}

impl ViewSetup for PositionView {
    fn setup(mut self: Weak<Self>) {
        self.enable_touch();
        self.place().size(250, 50);
        self.dot.set_color(Color::BLACK).place().tl(-5).size(10, 10);
        self.label.set_text("Move me").place().back();
        self.touch.began.val(move |touch| {
            self.began_pos = touch.position;
        });
        self.touch.moved.val(move |touch| {
            let new_pos = self.frame.origin + touch.position - self.began_pos;
            let mut label = format!("{:.0} - {:.0}", new_pos.x, new_pos.y);

            if let Some(additional_label) = &self.additional_label {
                label = format!("{additional_label} {label}");
            }

            self.label.set_text(label);
            self.set_position(new_pos);
            self.moved.trigger(new_pos);
        });
    }
}
