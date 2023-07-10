use gm::Color;
use rtools::IntoF32;

use crate::{PathData, View};

pub trait ViewData {
    fn color(&self) -> &Color;
    fn set_color(&mut self, color: impl Into<Color>) -> &mut Self;
    fn border_color(&self) -> &Color;
    fn set_border_color(&mut self, color: Color) -> &mut Self;
    fn corner_radius(&self) -> f32;
    fn set_corner_radius(&mut self, radius: impl IntoF32) -> &mut Self;
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

    fn paths(&self) -> &[PathData] {
        &self.paths
    }
}
