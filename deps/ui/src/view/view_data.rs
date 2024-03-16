use gm::{Color, IntoF32};
use refs::Weak;
use vents::Event;

use crate::{layout::Placer, NavigationView, UIAnimation, View};

pub trait ViewData {
    fn color(&self) -> &Color;
    fn set_color(&mut self, color: impl Into<Color>) -> &mut Self;

    fn border_color(&self) -> &Color;
    fn set_border_color(&mut self, color: Color) -> &mut Self;

    fn corner_radius(&self) -> f32;
    fn set_corner_radius(&mut self, radius: impl IntoF32) -> &mut Self;

    fn is_hidden(&self) -> bool;
    fn set_hidden(&mut self, is_hidden: bool) -> &mut Self;

    fn place(&self) -> &Placer;

    fn navigation_view(&self) -> Weak<NavigationView>;
    fn set_navigation_view(&mut self, nav: Weak<NavigationView>) -> &mut Self;

    fn label(&self) -> &str;
    fn set_label(&mut self, label: impl ToString) -> &mut Self;

    fn animations(&mut self) -> &mut Vec<UIAnimation>;

    fn dont_hide(&self) -> bool;

    fn loaded(&self) -> &Event;
    fn position_changed(&self) -> &Event;
    fn size_changed(&self) -> &Event;
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
    fn is_hidden(&self) -> bool {
        self.base().is_hidden
    }

    fn set_hidden(&mut self, is_hidden: bool) -> &mut Self {
        self.base_mut().is_hidden = is_hidden;
        self
    }

    fn place(&self) -> &Placer {
        assert!(
            self.base().placer.is_ok(),
            "Placer is not initialized yet. Place view only after it was added on superview."
        );
        &self.base().placer
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

    fn set_label(&mut self, label: impl ToString) -> &mut Self {
        self.base_mut().label = label.to_string();
        self
    }

    fn animations(&mut self) -> &mut Vec<UIAnimation> {
        &mut self.base_mut().animations
    }

    fn dont_hide(&self) -> bool {
        self.base().dont_hide
    }

    fn loaded(&self) -> &Event {
        &self.base().loaded
    }

    fn position_changed(&self) -> &Event {
        &self.base().position_changed
    }

    fn size_changed(&self) -> &Event {
        &self.base().size_changed
    }
}
