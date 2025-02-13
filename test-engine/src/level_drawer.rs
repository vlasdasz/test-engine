use level::LevelManager;
use manage::{ExistsManaged, data_manager::DataManager};
use refs::MainLock;
use render::{SpriteBoxPipepeline, TexturedSpriteBoxPipeline, rect_instance::RectInstance};
use ui::UIManager;
use wgpu::RenderPass;
use window::{SpriteView, Window};

static SPRITE_DRAWER: MainLock<SpriteBoxPipepeline> = MainLock::new();
static TEXTURED_SPRITE_DRAWER: MainLock<TexturedSpriteBoxPipeline> = MainLock::new();

pub(crate) struct LevelDrawer;

impl LevelDrawer {
    pub(crate) fn update() {
        LevelManager::update();
    }

    pub(crate) fn draw(pass: &mut RenderPass) {
        if LevelManager::no_level() {
            return;
        }
        let resolution = UIManager::resolution();

        let drawer = Window::drawer();
        let level = LevelManager::level();
        let camera_pos = *LevelManager::camera_pos();
        let scale = *LevelManager::scale();

        if level.background.is_ok() {
            drawer.background.draw(
                pass,
                level.background.get_static(),
                resolution,
                camera_pos.neg() / 10.0,
                0.0,
                scale,
            );
        }

        drawer.polygon.clear();

        for sprite in level.sprites() {
            if sprite.image.exists_managed() {
                TEXTURED_SPRITE_DRAWER.get_mut().add_with_image(
                    RectInstance {
                        size:       sprite.render_size(),
                        position:   sprite.position(),
                        color:      *sprite.color(),
                        rotation:   sprite.rotation(),
                        z_position: sprite.z_position,
                    },
                    sprite.image,
                );
            } else if let Some(vertex_buffer) = &sprite.vertex_buffer {
                drawer.polygon.add(
                    vertex_buffer,
                    sprite.position(),
                    *sprite.color(),
                    sprite.rotation(),
                );
            } else {
                SPRITE_DRAWER.get_mut().add(RectInstance {
                    size:       sprite.render_size(),
                    position:   sprite.position(),
                    color:      *sprite.color(),
                    rotation:   sprite.rotation(),
                    z_position: sprite.z_position,
                });
            }
        }

        SPRITE_DRAWER.get_mut().draw(
            pass,
            SpriteView {
                camera_pos,
                resolution,
                camera_rotation: 0.0,
                scale,
            },
        );
        TEXTURED_SPRITE_DRAWER.get_mut().draw(
            pass,
            SpriteView {
                camera_pos,
                resolution,
                camera_rotation: 0.0,
                scale,
            },
        );

        drawer.polygon.draw(
            pass,
            SpriteView {
                camera_pos,
                resolution,
                camera_rotation: 0.0,
                scale,
            },
        );
    }
}
