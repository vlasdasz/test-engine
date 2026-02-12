use std::cell::RefCell;

use ::test_engine::{
    refs::Weak,
    ui::{Label, NumberView, Setup, view},
};
use test_engine::{
    gm::LossyConvert,
    refs::Own,
    ui::{Anchor, TextAlignment, ViewData},
};

use crate::interface::test_game_view::Node;

pub struct Function<In, Out> {
    fun: RefCell<Box<dyn FnMut(In) -> Out>>,
}

impl<In, Out> Default for Function<In, Out> {
    fn default() -> Self {
        Self {
            fun: RefCell::new(Box::new(|_| panic!())),
        }
    }
}

impl<In, Out> Function<In, Out> {
    pub fn new(fun: impl FnMut(In) -> Out + 'static) -> Self {
        Self {
            fun: RefCell::new(Box::new(fun)),
        }
    }

    pub fn call(&self, input: In) -> Out {
        (*self.fun.borrow_mut())(input)
    }
}

#[view]
pub struct ScaleCell {
    get_scale: Function<(), f32>,
    set_scale: Function<f32, ()>,

    #[init]
    number: NumberView,
    label:  Label,
}

impl ScaleCell {
    pub fn make(get: Function<(), f32>, set: Function<f32, ()>) -> Own<Self> {
        let mut new = Self::new();
        new.get_scale = get;
        new.set_scale = set;
        new
    }

    pub fn set_node(self: Weak<Self>, node: &Node) {
        self.label
            .place()
            .clear()
            .l(10.0 + node.depth.lossy_convert() * 28.0)
            .tb(2.5)
            .anchor(Anchor::Right, self.number, 5);
    }
}

impl Setup for ScaleCell {
    fn setup(self: Weak<Self>) {
        self.place().distribute_ratio([4, 1]);

        self.number.place().trb(2.5).w(20);

        let scale = self.get_scale.call(());

        self.label
            .set_alignment(TextAlignment::Left)
            .set_text(format!("Scale: {scale:.2}"));
        self.number.set_min(0.2).set_step(0.2).set_value(scale);
        self.number.on_change(move |scale| {
            self.set_scale.call(scale);
            self.label.set_text(format!("Scale: {scale:.2}"));
        });
    }
}
