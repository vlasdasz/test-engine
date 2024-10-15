use scene::Scene;
use ui::UIManager;

pub(crate) struct SceneDrawer {}

impl SceneDrawer {
    pub(crate) fn update(scene: &mut Scene) {
        scene.set_aspect_ratio(UIManager::resolution());
    }
}
