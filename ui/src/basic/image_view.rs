use gl_image::Image;
use proc_macro::Boxed;

use crate::{View, ViewBase};

#[derive(Boxed)]
pub struct ImageView {
    pub image: Image,
    base:      ViewBase,
}

impl View for ImageView {
    fn image(&self) -> Option<Image> { self.image.into() }

    fn view(&self) -> &ViewBase { &self.base }

    fn view_mut(&mut self) -> &mut ViewBase { &mut self.base }
}
