use gm::Color;
use refs::MainLock;
use render::PathData;
use window::RenderPass;

use crate::pipelines::PATH;

static PATH_DATA: MainLock<Option<PathData>> = MainLock::new();

pub(crate) fn render_path(pass: &mut RenderPass) {
    let path = PATH_DATA.get_mut().get_or_insert_with(|| {
        PathData::new(
            Color::BLUE,
            (400, 400).into(),
            &[
                (0, 0).into(),
                (80, 100).into(),
                (20, 200).into(),
                (150, 20).into(),
                (20, 50).into(),
            ],
        )
    });

    PATH.draw(
        pass,
        &(100, 100, 280, 280).into(),
        path.buffer(),
        path.bind(),
        path.vertex_range(),
        0.5,
    );
}
