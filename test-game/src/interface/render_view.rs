use test_engine::{
    refs::Weak,
    ui::{view, Color, NumberView, UIManager, ViewCallbacks, ViewData, ViewSetup},
    RenderPass, SpriteView, WGPUApp,
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

        drawer.sprite_box.add((2, 2).into(), (0, 0).into(), 0.0, Color::RED);
        drawer.sprite_box.add((2, 2).into(), (40, 0).into(), 0.0, Color::GREEN);
        drawer.sprite_box.add((2, 2).into(), (40, 40).into(), 0.0, Color::BLUE);
        drawer.sprite_box.add((2, 2).into(), (0, 40).into(), 0.0, Color::TURQUOISE);
        drawer.sprite_box.draw(pass, 1.0, 0.0, (0, 0).into(), UIManager::resolution());

        drawer.polygon.add(
            vec![(0, 0).into(), (20, 0).into(), (0, 20).into(), (20, 20).into()],
            (-20, -20).into(),
            Color::GREEN,
        );

        drawer.polygon.add(
            vec![(0, 0).into(), (40, 0).into(), (0, 40).into(), (40, 40).into()],
            (10, 10).into(),
            Color::BLUE,
        );

        drawer.polygon.draw(
            pass,
            SpriteView {
                camera_pos:      Default::default(),
                resolution:      UIManager::resolution(),
                camera_rotation: 0.0,
                scale:           1.0,
            },
        );
    }
}
