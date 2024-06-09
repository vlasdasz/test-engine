use test_engine::{
    ui::{view, Color, Rect, ViewCallbacks},
    RenderPass, WGPUApp,
};

#[view]
pub struct RenderView {}

impl ViewCallbacks for RenderView {
    fn render<'a>(&self, pass: &mut RenderPass) {
        let drawer = WGPUApp::drawer();

        drawer
            .rect_drawer
            .draw(pass, &Rect::new(200.0, 500.0, 100.0, 150.0), &Color::GREEN, 0.5);
    }
}
