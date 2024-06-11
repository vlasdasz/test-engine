use test_engine::{
    refs::Weak,
    ui::{view, Image, NumberView, ViewCallbacks, ViewData, ViewSetup},
    DataManager, RenderPass, WGPUApp,
};

#[view]
pub struct RenderView {
    #[init]
    val: NumberView<f32>,
}

impl ViewSetup for RenderView {
    fn setup(mut self: Weak<Self>) {
        self.val.set_step(0.1).place().size(50, 100).bl(0);
    }
}

impl ViewCallbacks for RenderView {
    fn render(&self, pass: &mut RenderPass) {
        let drawer = WGPUApp::drawer();

        let image = Image::get("sky.png");

        drawer
            .test_pipeline
            .draw(pass, image.get_static(), (self.val.value(), 0).into(), 0.5);
    }
}
