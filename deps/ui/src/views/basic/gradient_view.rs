use gm::Color;
use ui_proc::view;

mod test_engine {
    pub(crate) use educe;
    pub(crate) use refs;

    pub(crate) use crate as ui;
}

#[view]
pub struct GradientView {
    pub start_color: Color,
    pub end_color:   Color,
}

impl GradientView {
    pub fn set_gradient(&mut self, start: impl Into<Color>, end: impl Into<Color>) -> &mut Self {
        self.start_color = start.into();
        self.end_color = end.into();
        self
    }
}
