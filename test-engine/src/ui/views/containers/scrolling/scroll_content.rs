use gm::flat::Size;
use refs::Own;
use ui::{View, ViewCallbacks, view};

use crate::{self as test_engine};

#[view]
pub(crate) struct ScrollContent {
    pub(super) content_size: Size,
}

impl ScrollContent {
    pub(crate) fn scroll_content_subviews(&self) -> &[Own<dyn View>] {
        &self.__view_base.__subviews()
    }

    pub(super) fn content_offset_mut(&mut self) -> &mut f32 {
        &mut self.__view_base.__content_offset
    }
}

impl ViewCallbacks for ScrollContent {
    fn content_size(&self) -> &Size {
        &self.content_size
    }
}
