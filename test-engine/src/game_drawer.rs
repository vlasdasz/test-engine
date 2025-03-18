use game::Game;
use gm::{Color, flat::Point};
use manage::data_manager::DataManager;
use refs::MainLock;
use render::{BackgroundPipeline, SpriteView, TexturedSpriteBoxPipeline, data::RectInstance};
use ui::UIManager;
use window::RenderPass;

static OBJECT_DRAWER: MainLock<TexturedSpriteBoxPipeline> = MainLock::new();
static BACKGROUND: MainLock<BackgroundPipeline> = MainLock::new();

pub struct GameDrawer;

impl GameDrawer {
    pub fn draw(pass: &mut RenderPass, game: &mut Game) {
        game.update();

        BACKGROUND.get_mut().draw(
            pass,
            game.background.get_static(),
            UIManager::window_resolution(),
            Point::default(),
            0.0,
            1.0,
        );

        for object in &game.objects {
            OBJECT_DRAWER.get_mut().add_with_image(
                RectInstance {
                    position:   object.position,
                    size:       object.size,
                    color:      Color::default(),
                    rotation:   object.rotation,
                    z_position: 0.85,
                },
                object.image,
            );
        }

        OBJECT_DRAWER.get_mut().draw(
            pass,
            SpriteView {
                camera_pos:      Point::default(),
                resolution:      UIManager::window_resolution(),
                camera_rotation: 0.0,
                scale:           1.0,
            },
        );
    }
}
