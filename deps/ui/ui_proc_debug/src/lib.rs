#![allow(incomplete_features)]
#![allow(clippy::single_component_path_imports)]
#![feature(arbitrary_self_types)]
#![feature(specialization)]

use ui::{view, SubView};
use ui_views::Button;

mod test_engine {
    pub(crate) mod ui {
        pub(crate) use ::ui::*;
    }
    pub(crate) use refs;
}

#[view]
struct ProcView {
    #[link = sokol]
    bete: SubView<Button>,
}

impl ProcView {
    fn sokol(self: refs::Weak<Self>) {
        dbg!(self.bete);
    }
}
