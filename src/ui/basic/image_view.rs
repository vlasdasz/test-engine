use crate::gm::{Color, Rect};
use crate::image::Image;
use crate::ui::input::Touch;
use crate::ui::view::View;
use crate::ui::ViewBase;
use std::any::Any;
use tools::refs::{DynWeak, MutWeak, Shared};
use tools::weak_self::HasWeakSelf;
use tools::{AsAny, New};

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

impl New for ImageView {
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
    fn color(&self) -> &Color {
        self.base.color()
    }
    fn set_color(&mut self, color: Color) {
        self.base.set_color(color)
    }

    fn touch_enabled(&self) -> bool {
        self.base.touch_enabled()
    }
    fn enable_touch(&mut self) {
        self.base.enable_touch()
    }

    fn frame(&self) -> &Rect {
        self.base.frame()
    }
    fn set_frame(&mut self, frame: Rect) {
        self.base.set_frame(frame)
    }

    fn absolute_frame(&self) -> &Rect {
        self.base.absolute_frame()
    }
    fn calculate_absolute_frame(&mut self) {
        self.base.calculate_absolute_frame()
    }

    fn superview(&self) -> DynWeak<dyn View> {
        self.base.superview()
    }

    fn set_superview(&mut self, superview: DynWeak<dyn View>) {
        self.base.set_superview(superview)
    }

    fn subviews(&self) -> &[Shared<dyn View>] {
        self.base.subviews()
    }

    fn add_subview(&mut self, view: Shared<dyn View>) {
        self.base.add_subview(view)
    }

    fn remove_all_subviews(&mut self) {
        self.base.remove_all_subviews()
    }

    fn check_touch(&self, touch: &mut Touch) {
        self.base.check_touch(touch)
    }
}
