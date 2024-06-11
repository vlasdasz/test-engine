use level::LevelManager;
use manage::ExistsManaged;
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

        for sprite in LevelManager::level_weak().sprites() {
            if sprite.image.exists_managed() {
                drawer.textured_sprite.add_instance(
                    sprite.image,
                    sprite.size(),
                    sprite.position(),
                    sprite.rotation(),
                    *sprite.color(),
                );
            } else {
                drawer.colored_sprite.add_instance(
                    sprite.size(),
                    sprite.position(),
                    sprite.rotation(),
                    *sprite.color(),
                );
            }
        }

        drawer.colored_sprite.draw(
            pass,
            *LevelManager::scale(),
            0.0,
            *LevelManager::camera_pos(),
            resolution,
        );
        drawer.textured_sprite.draw(
            pass,
            *LevelManager::scale(),
            0.0,
            *LevelManager::camera_pos(),
            resolution,
        );
    }
}
