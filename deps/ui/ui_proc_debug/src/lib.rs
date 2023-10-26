#![feature(arbitrary_self_types)]

use anyhow::Result;
use refs::Weak;
use ui::{view, Container, SubView};

#[view]
struct ProcView {
    #[link(sokol)]
    bete: SubView<Container>,
}

impl ProcView {
    async fn sokol(self: Weak<Self>) -> Result<()> {
        Ok(())
    }
}
