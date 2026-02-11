use test_engine::{
    gm::LossyConvert,
    refs::Weak,
    ui::{Anchor::Left, Button, Label, UIImages, ViewData, view},
};

use crate::interface::test_game_view::Node;

#[view]
pub struct NodeCell {
    #[init]
    button: Button,
    label:  Label,
}

impl NodeCell {
    pub fn set_node(self: Weak<Self>, node: &Node) {
        self.button.set_image(if node.open {
            UIImages::down()
        } else {
            UIImages::right()
        });

        self.label.set_text(&node.value);

        self.button
            .place()
            .clear()
            .l(10.0 + node.depth.lossy_convert() * 40.0)
            .center_y()
            .size(if node.is_leaf() { 0 } else { 20 }, 20);
        self.label.place().clear().anchor(Left, self.button, 10).trb(5);
    }
}
