use std::ops::DerefMut;

use test_engine::{
    RenderPass,
    game::{Game, GameDrawer, Object},
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

        self.game.background = Image::get("sky.png");

        self.game.objects.push(Own::new(Object {
            position: Point::default(),
            size:     (5, 10).into(),
            rotation: 0.0,
            image:    Image::get("cat.png"),
            velocity: (0.01, 0.01).into(),
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
                     198  124 - 154 189 230
                     173  343 - 139 177 214
                     385  352 - 129 183 231
                     395   83 - 191 215 238
                ",
        )?;

        // test_engine::ui_test::record_ui_test();

        Ok(())
    }
}
