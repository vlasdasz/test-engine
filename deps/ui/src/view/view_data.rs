use gm::Color;
use refs::Weak;
use rtools::IntoF32;

use crate::{layout::Placer, NavigationView, UIAnimation, View};

pub trait ViewData {
    fn color(&self) -> &Color;
    fn set_color(&mut self, color: impl Into<Color>) -> &mut Self;

    fn border_color(&self) -> &Color;
    fn set_border_color(&mut self, color: Color) -> &mut Self;

    fn corner_radius(&self) -> f32;
    fn set_corner_radius(&mut self, radius: impl IntoF32) -> &mut Self;

    fn is_selected(&self) -> bool;
    fn set_selected(&mut self, selected: bool) -> &mut Self;

    fn is_hidden(&self) -> bool;
    fn set_hidden(&mut self, is_hidden: bool) -> &mut Self;

    fn place(&self) -> &Placer;

    fn navigation_view(&self) -> Weak<NavigationView>;
    fn set_navigation_view(&mut self, nav: Weak<NavigationView>) -> &mut Self;

    fn label(&self) -> &str;

    fn animations(&mut self) -> &Vec<UIAnimation>;
}

impl<T: ?Sized + View> ViewData for T {
    fn color(&self) -> &Color {
        &self.base().color
    }

    fn set_color(&mut self, color: impl Into<Color>) -> &mut Self {
        self.base_mut().color = color.into();
        self
    }

    fn border_color(&self) -> &Color {
        &self.base().border_color
    }

    fn set_border_color(&mut self, color: Color) -> &mut Self {
        self.base_mut().border_color = color;
        self
    }

    fn corner_radius(&self) -> f32 {
        self.base().corner_radius
    }

    fn set_corner_radius(&mut self, radius: impl IntoF32) -> &mut Self {
        self.base_mut().corner_radius = radius.into_f32();
        self
    }

    fn is_selected(&self) -> bool {
        self.base().is_selected
    }

    fn set_selected(&mut self, is_selected: bool) -> &mut Self {
        self.base_mut().is_selected = is_selected;
        self
    }

    fn is_hidden(&self) -> bool {
        self.base().is_hidden
    }

    fn set_hidden(&mut self, is_hidden: bool) -> &mut Self {
        self.base_mut().is_hidden = is_hidden;
        self
    }

    fn place(&self) -> &Placer {
        &self.base().place
    }

    fn navigation_view(&self) -> Weak<NavigationView> {
        self.base().navigation_view
    }

    fn set_navigation_view(&mut self, nav: Weak<NavigationView>) -> &mut Self {
        self.base_mut().navigation_view = nav;
        self
    }

    fn label(&self) -> &str {
        &self.base().label
    }

    fn animations(&mut self) -> &Vec<UIAnimation> {
        &mut self.base_mut().animations
    }
}
