use level::LevelManager;
use manage::{data_manager::DataManager, ExistsManaged};
use ui::UIManager;
use wgpu::RenderPass;
use wgpu_wrapper::WGPUApp;

pub(crate) struct TELevel;

impl TELevel {
    pub(crate) fn update() {
        LevelManager::update(WGPUApp::current().frame_time());
    }

    pub(crate) fn draw(pass: &mut RenderPass) {
        if LevelManager::no_level() {
            return;
        }
        let resolution = UIManager::window_size();

        let drawer = WGPUApp::drawer();
        let level = LevelManager::level();
        let camera_pos = *LevelManager::camera_pos();
        let scale = *LevelManager::scale();

        if level.background.is_ok() {
            drawer.background.draw(
                pass,
                level.background.get_static(),
                resolution,
                camera_pos.neg() / 10,
                0.0,
                scale,
            );
        }

        for sprite in level.sprites() {
            if sprite.image.exists_managed() {
                drawer.textured_sprite.add_box(
                    sprite.image,
                    sprite.size(),
                    sprite.position(),
                    sprite.rotation(),
                    *sprite.color(),
                );
            } else {
                drawer.colored_sprite.add_box(
                    sprite.size(),
                    sprite.position(),
                    sprite.rotation(),
                    *sprite.color(),
                );
            }
        }

        drawer.colored_sprite.draw(pass, scale, 0.0, camera_pos, resolution);

        drawer.textured_sprite.draw(pass, scale, 0.0, camera_pos, resolution);
    }
}
