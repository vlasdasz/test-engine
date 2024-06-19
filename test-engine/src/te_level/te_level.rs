use level::LevelManager;
use manage::{data_manager::DataManager, ExistsManaged};
use ui::UIManager;
use wgpu::RenderPass;
use wgpu_wrapper::{SpriteView, WGPUApp};

pub(crate) struct TELevel;

impl TELevel {
    pub(crate) fn update() {
        LevelManager::update(WGPUApp::current().frame_time());
    }

    pub(crate) fn draw(pass: &mut RenderPass) {
        if LevelManager::no_level() {
            return;
        }
        let resolution = UIManager::resolution();

        let drawer = WGPUApp::drawer();
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
                drawer.textured_box.add_box(
                    sprite.image,
                    sprite.render_size(),
                    sprite.position(),
                    sprite.rotation(),
                    *sprite.color(),
                );
            } else if let Some(vertex_buffer) = &sprite.vertex_buffer {
                drawer.polygon.add(
                    vertex_buffer,
                    sprite.position(),
                    *sprite.color(),
                    sprite.rotation(),
                );
            } else {
                drawer.sprite_box.add(
                    sprite.render_size(),
                    sprite.position(),
                    sprite.rotation(),
                    *sprite.color(),
                );
            }
        }

        drawer.sprite_box.draw(pass, scale, 0.0, camera_pos, resolution);
        drawer.textured_box.draw(pass, scale, 0.0, camera_pos, resolution);

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
