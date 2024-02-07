use gm::Color;
use refs::Own;
use ui_proc::view;

use crate::{layout::Placer, View};
mod test_engine {
    pub(crate) use refs;

    pub(crate) use crate as ui;
}

#[view]
pub struct Container {}

impl Container {
    pub fn make_root_view() -> Own<Self> {
        let mut root = Own::<Container>::default();
        root.label = "Root view".to_string();
        root.color = Color::WHITE;
        let weak_root = root.weak_view();
        root.base_mut().placer = Placer::new(weak_root);
        root
    }
}
