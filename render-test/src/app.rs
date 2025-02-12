use gm::Color;
use refs::MainLock;
use render::{UIRectPipepeline, rect_instance::RectInstance, rect_view::RectView};
use window::{App, RenderPass, Window};

static PIPELINE: MainLock<UIRectPipepeline> = MainLock::new();

pub struct RenderApp {}

impl App for RenderApp {
    fn update(&mut self) {}

    fn render<'a>(&'a mut self, pass: &mut RenderPass<'a>) {
        let pipeline = PIPELINE.get_mut();

        pipeline.add(RectInstance::new((100, 100, 100, 100).into(), Color::BLACK, 0.5));

        pipeline.draw(
            pass,
            RectView {
                resolution: Window::current().size,
            },
        )
    }
}
