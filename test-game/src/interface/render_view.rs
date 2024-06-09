use test_engine::{
    refs::Weak,
    ui::{view, ViewCallbacks, ViewSetup},
};

#[view]
pub struct RenderView {}

impl ViewSetup for RenderView {
    fn setup(self: Weak<Self>) {}
}

impl ViewCallbacks for RenderView {
    fn update(&mut self) {}
}
