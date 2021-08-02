use crate::{View, ViewBase};
use gl_image::Image;
use proc_macro::AsAny;
use proc_macro::New;
use tools::refs::MutWeak;

#[derive(Debug, AsAny, New)]
pub struct ImageView {
    pub image: Image,
    base: ViewBase,
    _weak: MutWeak<ImageView>,
}

impl View for ImageView {
    fn image(&self) -> Option<Image> {
        self.image.into()
    }

    fn view(&self) -> &ViewBase {
        &self.base
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }
}
