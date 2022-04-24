use gl_image::Image;
use gm::Color;
use rtools::data_manager::Handle;

use crate::{complex::PathData, View};

pub trait ViewData {
    fn color(&self) -> Color;
    fn set_color(&mut self, color: Color) -> &mut Self;
    fn image(&self) -> Handle<Image>;
    fn set_image(&mut self, image: Handle<Image>) -> &mut Self;
    fn is_hidden(&self) -> bool;
    fn set_hidden(&mut self, hidden: bool) -> &mut Self;
    fn paths(&self) -> &[PathData];
}

impl<T: ?Sized + View> ViewData for T {
    fn color(&self) -> Color {
        self.view().color
    }

    fn set_color(&mut self, color: Color) -> &mut Self {
        self.view_mut().color = color;
        self
    }

    fn image(&self) -> Handle<Image> {
        self.view().image
    }

    fn set_image(&mut self, image: Handle<Image>) -> &mut Self {
        self.view_mut().image = image;
        self
    }

    fn is_hidden(&self) -> bool {
        self.view().is_hidden
    }

    fn set_hidden(&mut self, hidden: bool) -> &mut Self {
        self.view_mut().is_hidden = hidden;
        self
    }

    fn paths(&self) -> &[PathData] {
        &self.view().paths
    }
}
