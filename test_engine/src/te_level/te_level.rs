use gm::flat::Point;
use level::LevelManager;
use ui::UIManager;
use wgpu::RenderPass;
use wgpu_wrapper::WGPUApp;

pub(crate) struct TELevel;

impl TELevel {
    pub(crate) fn update() {
        LevelManager::update();
    }

    pub(crate) fn draw(pass: &mut RenderPass) {
        if LevelManager::no_level() {
            return;
        }
        let resolution = UIManager::window_size();

        let drawer = WGPUApp::drawer();

        for sprite in LevelManager::level_mut().sprites() {
            drawer.instanced_sprite_drawer.add_instance(
                sprite.size(),
                sprite.position(),
                sprite.rotation(),
                *sprite.color(),
            );
        }

        drawer.instanced_sprite_drawer.draw(pass, 0.0, Point::default(), resolution);
    }
}
