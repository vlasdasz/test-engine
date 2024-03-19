use gm::{
    flat::{Direction, Size},
    Apply,
};
use refs::Weak;
use ui_proc::view;
use vents::Event;

use crate::{
    view::{ViewData, ViewFrame},
    Sub, ViewCallbacks, ViewSetup, ViewTest,
};
mod test_engine {
    pub(crate) use refs;

    pub(crate) use crate as ui;
}

use crate::{Button, UIImages};

#[view]
pub struct DPadView {
    up:           Sub<Button>,
    down:         Sub<Button>,
    left:         Sub<Button>,
    right:        Sub<Button>,
    pub on_press: Event<Direction>,
}

impl ViewSetup for DPadView {
    fn setup(self: Weak<Self>) {
        [
            (self.up, Direction::Up, UIImages::up()),
            (self.down, Direction::Down, UIImages::down()),
            (self.left, Direction::Left, UIImages::left()),
            (self.right, Direction::Right, UIImages::right()),
        ]
        .apply(|(mut view, direction, image)| {
            view.set_image(image);
            view.on_tap(move || self.on_press.trigger(direction));
            view.set_corner_radius(5);
        });
    }
}

impl ViewCallbacks for DPadView {
    fn update(&mut self) {
        let width = self.width() / 3.0;
        let height = self.height() / 2.0;

        self.up.set_frame((width, 0, width, height));
        self.left.set_frame((0, height, width, height));
        self.down.set_frame((width, height, width, height));
        self.right.set_frame((width * 2.0, height, width, height));
    }
}

impl ViewTest for DPadView {
    fn test_size() -> Size<u32>
    where Self: Sized {
        (140, 100).into()
    }
}
