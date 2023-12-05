use gm::flat::IntSize;
use ui_proc::view;

use crate as ui;
use crate::ViewTest;

#[view]
pub struct Container {}

impl ViewTest for Container {
    fn test_size() -> IntSize
    where Self: Sized {
        (600, 600).into()
    }
}
