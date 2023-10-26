#![allow(incomplete_features)]
#![feature(arbitrary_self_types)]
#![feature(specialization)]

use anyhow::Result;
use refs::Weak;
use ui::{view, SubView};
use ui_views::Button;

#[view]
struct ProcView {
    #[link = sokol]
    bete: SubView<Button>,
}

impl ProcView {
    fn sokol(self: Weak<Self>) -> Result<()> {
        dbg!("A");
        Ok(())
    }
}
