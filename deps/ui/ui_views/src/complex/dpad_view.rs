use gl_image::Image;
use gm::flat::Direction;
use refs::ToWeak;
use rtools::{data_manager::Handle, Apply};
use ui::{view, SubView, ViewCallbacks, ViewData, ViewFrame, Event};

use crate::Button;

#[view]
#[derive(Default)]
pub struct DPadView {
    up:           SubView<Button>,
    down:         SubView<Button>,
    left:         SubView<Button>,
    right:        SubView<Button>,
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
        [self.up, self.down, self.left, self.right].apply2(
            [Direction::Up, Direction::Down, Direction::Left, Direction::Right],
            |view, direction| {
                let this = self.weak();
                view.on_tap.sub(move |_| this.on_press.trigger(direction));
                view.set_corner_radius(5);
            },
        );
    }

    fn update(&mut self) {
        let width = self.width() / 3.0;
        let height = self.height() / 2.0;

        self.up.set_frame((width, 0, width, height));
        self.left.set_frame((0, height, width, height));
        self.down.set_frame((width, height, width, height));
        self.right.set_frame((width * 2.0, height, width, height));
    }
}
