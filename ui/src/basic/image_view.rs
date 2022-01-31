use gl_image::Image;

use crate::{view_base::ViewBase, View};

#[derive(Default)]
pub struct ImageView {
    pub image: Image,
    base:      ViewBase,
}

impl View for ImageView {
    fn image(&self) -> Option<Image> {
        self.image.clone().into()
    }

    fn view(&self) -> &ViewBase {
        &self.base
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }
}
