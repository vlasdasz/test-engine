use crate::ui::view::{View, AsAny};
use crate::utils::{HasWeakSelf, Shared, MutWeak, make_shared, DynWeak};
use std::rc::Rc;
use crate::ui::ViewBase;
use crate::gm::{Color, Rect};
use crate::ui::input::Touch;
use std::any::Any;
use crate::image::Image;

pub struct ImageView {
    pub image: Image,
    base: ViewBase,
    _weak: MutWeak<ImageView>
}

impl AsAny for ImageView {
    fn as_any(&self) -> &dyn Any { self }
}

impl HasWeakSelf for ImageView {

    fn new() -> Self {
        ImageView { image: Image::new(), base: ViewBase::new(), _weak: MutWeak::new() }
    }

    fn new_shared() -> Shared<Self> {
        let result = make_shared(ImageView::new());
        result.try_borrow_mut().unwrap()._weak = Rc::downgrade(&result);
        result
    }

    fn weak(&self) -> MutWeak<Self> {
        self._weak.clone()
    }
}

impl View for ImageView {
    fn color(&self) -> &Color { self.base.color() }
    fn set_color(&mut self, color: Color) { self.base.set_color(color) }

    fn touch_enabled(&self) -> bool { self.base.touch_enabled() }
    fn enable_touch(&mut self) { self.base.enable_touch() }

    fn set_frame(&mut self, frame: Rect) { self.base.set_frame(frame) }

    fn absolute_frame(&self) -> &Rect { self.base.absolute_frame() }
    fn calculate_absolute_frame(&mut self) { self.base.calculate_absolute_frame() }

    fn superview(&self) -> DynWeak<dyn View> { self.base.superview() }
    fn set_superview(&mut self, superview: DynWeak<dyn View>) { self.base.set_superview(superview) }

    fn subviews(&self) -> &[Shared<dyn View>] { self.base.subviews() }

    fn check_touch(&self, touch: &mut Touch) { self.base.check_touch(touch) }
}