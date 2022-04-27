use test_engine::{
    audio::Sound,
    gl_wrapper::GLWrapper,
    gm::{
        flat::{Point, Shape},
        Color,
    },
    rtools::{
        data_manager::{DataManager, Handle},
        Rglica, ToRglica,
    },
    sprites::{Body, LevelCreation, Player, SpriteTemplates, Wall},
    Image, Level, LevelBase, Sprite,
};

#[derive(Default, Debug)]
pub struct TestGameLevel {
    base:            LevelBase,
    selected_sprite: Option<Rglica<dyn Sprite>>,
    collision_sound: Handle<Sound>,
    pub player:      Rglica<Player>,
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
            self.base_mut().on_sprite_selected.trigger(Rglica::default());
        }
    }
}

impl Level for TestGameLevel {
    fn setup(&mut self) {
        error!("setup TestGameLevel");

        let drawn = Image::draw((100, 100), |image| {
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

        self.add_sprite::<Wall>((100, 5), (0, 0)).set_image(square);
        self.add_sprite::<Wall>((5, 100), (60, 0)).set_image(square);
        self.add_sprite::<Wall>((5, 100), (-60, 0)).set_image(square);

        self.add_sprite::<Body>(Shape::triangle((-10, -10), (10, -10), (-10, 10)), (0, 50))
            .set_image(Image::get("triangle.png"));

        for i in 0..50 {
            self.add_sprite::<Body>((0.5, 0.5), (0.1 * i as f32, i * 2))
                .set_image(square);
        }

        self.player = self.add_sprite((2, 2), (0, 5));
        self.player
            .set_image(Image::get("frisk.png"))
            .enable_collision_detection();
        self.player.weapon.set_image(Image::get("ak.png"));
        self.player.on_collision.set(self, |_, this| {
            this.collision_sound.play();
        });

        self.collision_sound = Sound::get("pek.wav");

        self.base.on_tap.set(self, |pos, this| this.on_touch(pos));
    }

    fn update(&mut self) {
        let pos = self.player.position();
        self.set_camera_position(pos);
    }

    fn base(&self) -> &LevelBase {
        &self.base
    }

    fn base_mut(&mut self) -> &mut LevelBase {
        &mut self.base
    }

    fn rglica(&self) -> Rglica<dyn Level> {
        (self as &dyn Level).to_rglica()
    }
}
