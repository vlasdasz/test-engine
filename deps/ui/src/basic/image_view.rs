use gl_image::Image;
use rtools::data_manager::Handle;

use crate::{view_base::ViewBase, View};

#[derive(Default, Debug)]
pub struct ImageView {
    image: Handle<Image>,
    base:  ViewBase,
}

impl View for ImageView {
    fn image(&self) -> Handle<Image> {
        self.image
    }

    fn set_image(&mut self, image: Handle<Image>) {
        self.image = image
    }

    fn view(&self) -> &ViewBase {
        &self.base
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }
}
