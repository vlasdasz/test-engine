use gl_image::Image;
use gm::Color;
use rtools::{data_manager::Handle, IntoF32, Rglica};

use crate::{complex::PathData, UIDrawer, View};

pub trait ViewData {
    fn color(&self) -> &Color;
    fn set_color(&mut self, color: impl Into<Color>) -> &mut Self;
    fn border_color(&self) -> &Color;
    fn set_border_color(&mut self, color: Color) -> &mut Self;
    fn corner_radius(&self) -> f32;
    fn set_corner_radius(&mut self, radius: impl IntoF32) -> &mut Self;
    fn image(&self) -> Handle<Image>;
    fn set_image(&mut self, image: Handle<Image>) -> &mut Self;
    fn is_hidden(&self) -> bool;
    fn set_hidden(&mut self, hidden: bool) -> &mut Self;
    fn drawer(&self) -> Rglica<dyn UIDrawer>;
    fn set_drawer(&mut self, _: Rglica<dyn UIDrawer>);
    fn paths(&self) -> &[PathData];
}

impl<T: ?Sized + View> ViewData for T {
    fn color(&self) -> &Color {
        &self.color
    }

    fn set_color(&mut self, color: impl Into<Color>) -> &mut Self {
        self.color = color.into();
        self
    }

    fn border_color(&self) -> &Color {
        &self.border_color
    }

    fn set_border_color(&mut self, color: Color) -> &mut Self {
        self.border_color = color;
        self
    }

    fn corner_radius(&self) -> f32 {
        self.corner_radius
    }

    fn set_corner_radius(&mut self, radius: impl IntoF32) -> &mut Self {
        self.corner_radius = radius.into_f32();
        self
    }

    fn image(&self) -> Handle<Image> {
        self.image
    }

    fn set_image(&mut self, image: Handle<Image>) -> &mut Self {
        self.image = image;
        self
    }

    fn is_hidden(&self) -> bool {
        self.is_hidden
    }

    fn set_hidden(&mut self, hidden: bool) -> &mut Self {
        self.is_hidden = hidden;
        self
    }

    fn drawer(&self) -> Rglica<dyn UIDrawer> {
        self.drawer
    }

    fn set_drawer(&mut self, drawer: Rglica<dyn UIDrawer>) {
        self.drawer = drawer
    }

    fn paths(&self) -> &[PathData] {
        &self.paths
    }
}
