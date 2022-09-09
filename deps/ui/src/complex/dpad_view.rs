use gl_image::Image;
use gm::flat::Direction;
use rtools::{data_manager::Handle, Apply, Event, Rglica, ToRglica};

use crate::{
    basic::Button,
    view,
    view::{ViewData, ViewSubviews},
    View, ViewBase, ViewCallbacks,
};

#[view]
#[derive(Default, Debug)]
pub struct DPadView {
    up:           Rglica<Button>,
    down:         Rglica<Button>,
    left:         Rglica<Button>,
    right:        Rglica<Button>,
    pub on_press: Event<Direction>,
}

impl DPadView {
    pub fn set_images(
        &mut self,
        up: Handle<Image>,
        down: Handle<Image>,
        left: Handle<Image>,
        right: Handle<Image>,
    ) -> &mut Self {
        self.up.set_image(up);
        self.down.set_image(down);
        self.left.set_image(left);
        self.right.set_image(right);
        self
    }
}

impl ViewCallbacks for DPadView {
    fn setup(&mut self) {
        (self.up, self.down, self.left, self.right) =
            (self.add_view(), self.add_view(), self.add_view(), self.add_view());

        [self.up, self.down, self.left, self.right].apply2(
            [Direction::Up, Direction::Down, Direction::Left, Direction::Right],
            |view, direction| {
                view.on_tap
                    .set(self, move |this, _| this.on_press.trigger(direction));
                view.set_corner_radius(5);
            },
        );
    }
}
