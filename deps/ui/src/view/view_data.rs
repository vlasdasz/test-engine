use gl_image::Image;
use gm::Color;
use rtools::{data_manager::Handle, IntoF32, Rglica};

use crate::{complex::PathData, UIDrawer, View};

pub trait ViewData {
    fn color(&self) -> &Color;
    fn set_color(&mut self, color: Color) -> &mut Self;
    fn border_color(&self) -> &Color;
    fn set_border_color(&mut self, color: Color) -> &mut Self;
    fn corner_radius(&self) -> f32;
    fn set_corner_radius(&mut self, radius: impl IntoF32) -> &mut Self;
    fn image(&self) -> Handle<Image>;
    fn set_image(&mut self, image: Handle<Image>) -> &mut Self;
    fn is_hidden(&self) -> bool;
    fn set_hidden(&mut self, hidden: bool) -> &mut Self;
    fn drawer(&mut self) -> Rglica<dyn UIDrawer>;
    fn set_drawer(&mut self, _: Rglica<dyn UIDrawer>);
    fn paths(&self) -> &[PathData];
}

impl<T: ?Sized + View> ViewData for T {
    fn color(&self) -> &Color {
        &self.view().color
    }

    fn set_color(&mut self, color: Color) -> &mut Self {
        self.view_mut().color = color;
        self
    }

    fn border_color(&self) -> &Color {
        &self.view().border_color
    }

    fn set_border_color(&mut self, color: Color) -> &mut Self {
        self.view_mut().border_color = color;
        self
    }

    fn corner_radius(&self) -> f32 {
        self.view().corner_radius
    }

    fn set_corner_radius(&mut self, radius: impl IntoF32) -> &mut Self {
        self.view_mut().corner_radius = radius.into_f32();
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

    fn drawer(&mut self) -> Rglica<dyn UIDrawer> {
        self.view_mut().drawer
    }

    fn set_drawer(&mut self, drawer: Rglica<dyn UIDrawer>) {
        self.view_mut().drawer = drawer
    }

    fn paths(&self) -> &[PathData] {
        &self.view().paths
    }
}
