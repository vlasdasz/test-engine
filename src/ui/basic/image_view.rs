use crate::image::Image;

use crate::ui::view::View;
use crate::ui::ViewBase;
use std::any::Any;
use tools::refs::MutWeak;
use tools::weak_self::HasWeakSelf;
use tools::{AsAny, HasNew};

#[derive(Debug)]
pub struct ImageView {
    pub image: Image,
    base: ViewBase,
    _weak: MutWeak<ImageView>,
}

impl AsAny for ImageView {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl HasNew for ImageView {
    fn new() -> Self {
        Self {
            image: Image::new(),
            base: ViewBase::new(),
            _weak: MutWeak::new(),
        }
    }
}

impl HasWeakSelf for ImageView {
    fn weak(&self) -> MutWeak<Self> {
        self._weak.clone()
    }

    fn set_weak(&mut self, weak: MutWeak<Self>) {
        self._weak = weak
    }
}

impl View for ImageView {
    fn view(&self) -> &ViewBase {
        &self.base
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }
}
