use game::Game;
use gm::flat::Point;
use refs::{main_lock::MainLock, manage::DataManager};
use render::{BackgroundPipeline, SpriteView, TexturedSpriteBoxPipeline, data::TexturedSpriteInstance};
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
            unsafe { game.background.get_static() },
            UIManager::window_resolution(),
            Point::default(),
            0.0,
            1.0,
        );

        for object in &game.objects {
            OBJECT_DRAWER.get_mut().add_with_image(
                TexturedSpriteInstance {
                    position:   object.position,
                    size:       object.size,
                    scale:      1.0,
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
                _padding:        0,
            },
        );
    }
}
