use test_engine::{
    RenderPass, SpriteView, Window,
    refs::Weak,
    ui::{NumberView, Point, Setup, UIManager, ViewCallbacks, ViewData, ViewSubviews, view},
};

use crate::interface::test_game_view::TestGameView;

#[view]
pub struct RenderView {
    #[init]
    val: NumberView<f32>,
}

impl Setup for RenderView {
    fn setup(mut self: Weak<Self>) {
        self.val.set_step(0.1).place().size(50, 100).bl(0);

        self.add_transition::<Self, TestGameView>()
            .set_text("Back")
            .place()
            .size(100, 50)
            .t(200)
            .l(10);
    }
}

impl ViewCallbacks for RenderView {
    fn render(&self, pass: &mut RenderPass) {
        let drawer = Window::drawer();

        // drawer.sprite_box.add((2, 2).into(), (0, 0).into(), 0.0, Color::RED, 0.5);
        // drawer.sprite_box.add((2, 2).into(), (40, 0).into(), 0.0, Color::GREEN, 0.5);
        // drawer.sprite_box.add((2, 2).into(), (40, 40).into(), 0.0, Color::BLUE, 0.5);
        // drawer.sprite_box.add((2, 2).into(), (0, 40).into(), 0.0, Color::TURQUOISE,
        // 0.5); drawer.sprite_box.draw(pass, 1.0, 0.0, (0, 0).into(),
        // UIManager::resolution());

        drawer.polygon.clear();

        drawer.polygon.draw(pass, SpriteView {
            camera_pos:      Point::default(),
            resolution:      UIManager::resolution(),
            camera_rotation: 0.0,
            scale:           1.0,
        });
    }
}
