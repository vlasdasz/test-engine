use std::ops::DerefMut;

use test_engine::{
    DataManager, RenderPass,
    game::{Game, GameDrawer, Object},
    refs::{Own, Weak},
    ui::{Button, Image, Point, Setup, ViewCallbacks, ViewData, view},
};

use crate::interface::test_game_view::HAS_BACK_BUTTON;

#[view]
pub struct GameView {
    game: Own<Game>,

    #[init]
    back: Button,
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
