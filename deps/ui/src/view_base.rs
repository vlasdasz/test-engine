use derivative::Derivative;
use gm::{flat::Rect, Color};
use rtools::{IntoF32, Rglica};

use crate::{basic::Placer, view::ViewTemplates, View};

#[derive(Default, Derivative)]
#[derivative(Debug)]
pub struct ViewBase {
    pub(crate) color: Color,

    pub(crate) touch_enabled: bool,

    pub(crate) is_hidden: bool,

    pub(crate) frame:          Rect,
    #[derivative(Debug = "ignore")]
    pub(crate) absolute_frame: Rect,

    #[derivative(Debug = "ignore")]
    pub(crate) superview: Rglica<ViewBase>,
    #[derivative(Debug = "ignore")]
    pub(crate) subviews:  Vec<Box<dyn View>>,

    pub(crate) touch_id: u64,

    #[derivative(Debug = "ignore")]
    pub(crate) placer: Placer,
}

impl ViewBase {
    pub fn dummy() -> Box<Self> {
        let mut dummy = Self::default();
        dummy.set_frame((5, 5)).set_color(Color::random());
        Box::new(dummy)
    }
}

impl View for ViewBase {
    fn view(&self) -> &ViewBase {
        self
    }

    fn view_mut(&mut self) -> &mut Self {
        self
    }
}

impl<W: IntoF32, H: IntoF32> From<(W, H)> for Box<dyn View> {
    fn from(data: (W, H)) -> Self {
        Box::new(ViewBase {
            frame: (data.0, data.1).into(),
            ..Default::default()
        })
    }
}
