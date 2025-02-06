use level::LevelManager;
use manage::{ExistsManaged, data_manager::DataManager};
use refs::MainLock;
use render::SpriteBoxPipepeline;
use ui::UIManager;
use wgpu::RenderPass;
use window::{SpriteInstance, SpriteView, Window};

static SPRITE_DRAWER: MainLock<SpriteBoxPipepeline> = MainLock::new();

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
                drawer.textured_box.add(
                    sprite.image,
                    SpriteInstance {
                        size:       sprite.render_size(),
                        position:   sprite.position(),
                        color:      *sprite.color(),
                        rotation:   sprite.rotation(),
                        z_position: sprite.z_position,
                    },
                );
            } else if let Some(vertex_buffer) = &sprite.vertex_buffer {
                drawer.polygon.add(
                    vertex_buffer,
                    sprite.position(),
                    *sprite.color(),
                    sprite.rotation(),
                );
            } else {
                SPRITE_DRAWER.get_mut().add(SpriteInstance {
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
        drawer.textured_box.draw(
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
