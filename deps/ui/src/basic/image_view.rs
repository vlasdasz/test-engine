use gl_image::Image;

use crate::{view_base::ViewBase, View};

#[derive(Default, Debug)]
pub struct ImageView {
    image: Image,
    base:  ViewBase,
}

impl View for ImageView {
    fn image(&self) -> Option<Image> {
        self.image.clone().into()
    }

    fn set_image(&mut self, image: Image) {
        self.image = image
    }

    fn view(&self) -> &ViewBase {
        &self.base
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }
}
