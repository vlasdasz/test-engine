use rtools::data_manager::{DataManager, Handle};
use test_engine::{
    audio::Sound,
    gl_wrapper::GLWrapper,
    gm::{
        flat::{Point, Shape},
        Color,
    },
    sprites::{Body, LevelCreation, Player, SpriteTemplates, Wall},
    text::{render_text, Font},
    Image, Level, LevelBase, Sprite,
};
use ui::refs::{ToWeak, Weak};

#[derive(Default)]
pub struct TestGameLevel {
    base:            LevelBase,
    selected_sprite: Option<Weak<dyn Sprite>>,
    collision_sound: Handle<Sound>,
}

impl TestGameLevel {
    fn on_touch(&mut self, pos: Point) {
        if let Some(mut sprite) = self.sprite_at(pos) {
            sprite.set_selected(true);
            self.base_mut().on_sprite_selected.trigger(sprite);
            if let Some(mut old) = self.selected_sprite {
                old.set_selected(false);
            }
            self.selected_sprite = sprite.into();
            return;
        }

        if let Some(mut sprite) = self.selected_sprite {
            sprite.set_selected(false);
            self.selected_sprite = None;
            self.base_mut().on_sprite_selected.trigger(Weak::default());
        }
    }
}

impl Level for TestGameLevel {
    fn setup(&mut self) {
        let drawn = Image::draw("test_draw", (100, 100), |image| {
            GLWrapper::set_clear_color(Color::GREEN);
            GLWrapper::clear();
            GLWrapper::scissor((5, 5, 20, 20), || {
                GLWrapper::set_clear_color(Color::TURQUOISE);
                GLWrapper::clear();
            });
            GLWrapper::set_clear_color(Color::GRAY);
            image.channels = 1;
        });

        let square = Image::get("square.png");

        self.add_rect((30, 30, 40, 25)).set_image(drawn);

        self.add_sprite::<Wall>((100, 5), (0, 0)).set_image(render_text(
            "oo spolokolkok",
            &Font::san_francisco(),
            64,
        ));
        self.add_sprite::<Wall>((5, 100), (60, 0)).set_image(square);
        self.add_sprite::<Wall>((5, 100), (-60, 0)).set_image(square);

        self.add_sprite::<Body>(Shape::triangle((-10, -10), (10, -10), (-10, 10)), (0, 50))
            .set_image(Image::get("triangle.png"));

        for i in 0..500 {
            self.add_sprite::<Body>((0.5, 0.5), (0.1 * i as f32, i * 2)).set_image(square);
        }

        let mut player: Weak<Player> = self.add_sprite((2, 2), (0, 5));
        self.base_mut().player = player;
        player.set_image(Image::get("frisk.png")).enable_collision_detection();
        player.weapon.set_image(Image::get("ak.png"));
        let mut this = self.weak();
        player.on_collision.sub(move |_| {
            this.collision_sound.play();
        });

        self.collision_sound = Sound::get("pek.wav");

        self.base.on_tap.sub(move |pos| this.on_touch(pos));
    }

    fn update(&mut self) {
        let pos = self.player().position();
        self.set_camera_position(pos);
    }

    fn base(&self) -> &LevelBase {
        &self.base
    }

    fn base_mut(&mut self) -> &mut LevelBase {
        &mut self.base
    }

    fn rglica(&self) -> Weak<dyn Level> {
        (self as &dyn Level).weak()
    }
}
