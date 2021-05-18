use crate::gm::{Color, Rect};
use crate::ui::input::Touch;
use crate::ui::view::{View};
use crate::ui::{Font, ImageView, ViewBase};
use std::any::Any;
use std::rc::Rc;
use tools::refs::{make_shared, DynWeak, MutWeak, Shared};
use tools::weak_self::HasWeakSelf;
use tools::{New, AsAny};
use std::ops::{Deref, DerefMut};

pub struct Label {
    pub font: Font,
    base: ViewBase,
    _weak: MutWeak<Label>,
}

impl Label {
    pub fn set_text(&mut self, text: &str) {
        self.remove_all_subviews();

        if text.is_empty() {
            return;
        }

        let mut last_max_x: f32 = 0.0;
        let mut advance: f32 = 0.0;
        let mut content_size = self.base.frame().size;

        content_size.height = self.font.height;

        for letter in text.chars() {
            let glyph = self.font.glyph_for_char(letter);

            let mut glyph_view = ImageView::new();

            glyph_view.set_frame(Rect::from_size(&glyph.size()));
            glyph_view.image = glyph.image;

            glyph_view.set_frame(Rect::make(
                advance + glyph.bearing.x,
                content_size.height - glyph.bearing.y + self.font.baseline_shift,
                glyph.size().width,
                glyph.size().height,
            ));

            last_max_x = glyph_view.frame().max_x();

            advance += glyph.advance as f32;

            self.add_subview(make_shared(glyph_view));
        }

        content_size.width = last_max_x;

        self.set_frame(Rect::make(
            self.frame().origin.x,
            self.frame().origin.y,
            content_size.width,
            content_size.height,
        ));
    }
}

impl AsAny for Label {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl New for Label {
    fn new() -> Self {
        Self {
            font: Font::blank(),
            base: ViewBase::new(),
            _weak: MutWeak::new(),
        }
    }
}

impl HasWeakSelf for Label {
    fn new_shared() -> Shared<Self> {
        let result = make_shared(Self::new());
        result.try_borrow_mut().unwrap()._weak = Rc::downgrade(&result);
        result
    }

    fn weak(&self) -> MutWeak<Self> {
        self._weak.clone()
    }
}

impl Deref for Label {
    type Target = ViewBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for Label {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl View for Label {
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
