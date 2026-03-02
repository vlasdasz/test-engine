use std::ops::DerefMut;

use test_engine::{
    RenderPass,
    game::{Game, GameDrawer, Object, Shape},
    refs::{Own, Weak, manage::DataManager},
    ui::{Image, Point, Setup, ViewCallbacks, ViewData, ViewTest, view_test},
    ui_test::check_colors,
};

use crate::interface::test_game_view::HAS_BACK_BUTTON;

#[view_test]
pub struct GameView {
    game: Own<Game>,
}

impl Setup for GameView {
    fn setup(mut self: Weak<Self>) {
        self.apply_style(HAS_BACK_BUTTON);

        self.game.skybox = Image::get("sky.png");

        self.game.objects.push(Own::new(Object {
            position: Point::default(),
            rotation: 0.0,
            texture:  Image::get("cat.png"),
            velocity: (0.1, 0.1).into(),
            shape:    Shape::Rect((5, 10).into()),
        }));
    }
}

impl ViewCallbacks for GameView {
    fn before_render(&self, pass: &mut RenderPass) {
        GameDrawer::draw(pass, self.game.weak().deref_mut());
    }
}

impl ViewTest for GameView {
    fn perform_test(_view: Weak<Self>) -> anyhow::Result<()> {
        check_colors(
            r"
                     198  124 -  72 165  95
                ",
        )?;

        // test_engine::ui_test::record_ui_test();

        Ok(())
    }
}
