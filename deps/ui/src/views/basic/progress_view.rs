use gm::{ToF32, color::BLUE};
use refs::Weak;

use crate::{Container, Setup, ViewData, view};

mod test_engine {
    pub(crate) use educe;
    pub(crate) use refs;

    pub(crate) use crate as ui;
}

#[view]
pub struct ProgressView {
    #[init]
    pub bar: Container,
}

impl ProgressView {
    pub fn set_progress(self: Weak<Self>, progress: impl ToF32) {
        self.bar.place().clear().tlb(0).relative_width(self, progress);
    }
}

impl Setup for ProgressView {
    fn setup(self: Weak<Self>) {
        self.bar.set_color(BLUE).place().back();
    }
}
