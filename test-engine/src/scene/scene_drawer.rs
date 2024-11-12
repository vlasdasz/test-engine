use scene::Scene;
use ui::UIManager;

pub(crate) struct _SceneDrawer {}

impl _SceneDrawer {
    pub(crate) fn _update(scene: &mut Scene) {
        scene.set_aspect_ratio(UIManager::resolution());
    }
}
