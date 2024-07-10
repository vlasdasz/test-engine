mod test_engine {
    pub(crate) use educe;
    pub(crate) use refs;

    pub(crate) use crate as ui;
}
use gm::Color;
use ui_proc::view;

use crate::{has_data::HasText, Button, Container, ViewData, ViewSetup, ViewSubviews};

const A: &[char] = &['q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p'];
const B: &[char] = &['a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l'];
const C: &[char] = &['z', 'x', 'c', 'v', 'b', 'n', 'm'];

const LAYOUT: &[&[char]] = &[A, B, C];

#[view]
pub struct KeyboardView {}

impl ViewSetup for KeyboardView {
    fn setup(mut self: refs::Weak<Self>) {
        self.place().all_ver();

        for (i, row) in LAYOUT.iter().enumerate() {
            let mut container = self.add_view::<Container>();

            if i == 1 {
                container.place().lr(20);
            };
            if i == 2 {
                container.place().lr(80);
            };

            container.place().all_hor();
            for button in *row {
                container.add_view::<Button>().set_text(button).outline(Color::GRAY);
            }
        }
    }
}
