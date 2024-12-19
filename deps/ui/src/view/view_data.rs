use gm::{Color, ToF32};
use refs::{Own, Weak};
use vents::{Event, OnceEvent};

use crate::{NavigationView, UIAnimation, View, layout::Placer};

pub trait ViewData {
    fn tag(&self) -> usize;
    fn set_tag(&mut self, tag: usize) -> &mut Self;

    fn view_label(&self) -> &str;

    fn content_offset(&self) -> f32;

    fn color(&self) -> &Color;
    fn set_color(&mut self, color: impl Into<Color>) -> &mut Self;

    fn border_color(&self) -> &Color;
    fn set_border_color(&mut self, color: Color) -> &mut Self;

    fn corner_radius(&self) -> f32;
    fn set_corner_radius(&mut self, radius: impl ToF32) -> &mut Self;

    fn is_hidden(&self) -> bool;
    fn set_hidden(&mut self, is_hidden: bool) -> &mut Self;

    fn place(&self) -> &Placer;

    fn navigation_view(&self) -> Weak<NavigationView>;
    fn set_navigation_view(&mut self, nav: Weak<NavigationView>) -> &mut Self;

    fn label(&self) -> &str;
    fn set_label(&mut self, label: impl ToString) -> &mut Self;

    fn animations(&mut self) -> &mut Vec<UIAnimation>;

    fn dont_hide(&self) -> bool;

    fn position_changed(&self) -> &Event;
    fn size_changed(&self) -> &Event;

    fn __after_setup_event(&self) -> &OnceEvent;
}

impl<T: ?Sized + View> ViewData for T {
    fn tag(&self) -> usize {
        self.base_view().tag
    }

    fn set_tag(&mut self, tag: usize) -> &mut Self {
        self.base_view_mut().tag = tag;
        self
    }

    fn view_label(&self) -> &str {
        &self.base_view().view_label
    }

    fn content_offset(&self) -> f32 {
        self.base_view().content_offset
    }

    fn color(&self) -> &Color {
        &self.base_view().color
    }

    fn set_color(&mut self, color: impl Into<Color>) -> &mut Self {
        self.base_view_mut().color = color.into();
        self
    }

    fn border_color(&self) -> &Color {
        &self.base_view().border_color
    }

    fn set_border_color(&mut self, color: Color) -> &mut Self {
        self.base_view_mut().border_color = color;
        self
    }

    fn corner_radius(&self) -> f32 {
        self.base_view().corner_radius
    }

    fn set_corner_radius(&mut self, radius: impl ToF32) -> &mut Self {
        self.base_view_mut().corner_radius = radius.to_f32();
        self
    }
    fn is_hidden(&self) -> bool {
        self.base_view().is_hidden
    }

    fn set_hidden(&mut self, is_hidden: bool) -> &mut Self {
        self.base_view_mut().is_hidden = is_hidden;
        self
    }

    fn place(&self) -> &Placer {
        let placer = &self.base_view().placer;
        assert!(
            placer.is_ok(),
            "Invalid placer. Most likely this view was not initialized properly"
        );
        placer
    }

    fn navigation_view(&self) -> Weak<NavigationView> {
        self.base_view().navigation_view
    }

    fn set_navigation_view(&mut self, nav: Weak<NavigationView>) -> &mut Self {
        self.base_view_mut().navigation_view = nav;
        self
    }

    fn label(&self) -> &str {
        &self.base_view().view_label
    }

    fn set_label(&mut self, label: impl ToString) -> &mut Self {
        self.base_view_mut().view_label = label.to_string();
        self
    }

    fn animations(&mut self) -> &mut Vec<UIAnimation> {
        &mut self.base_view_mut().animations
    }

    fn dont_hide(&self) -> bool {
        self.base_view().dont_hide_off_screen
    }

    fn position_changed(&self) -> &Event {
        &self.base_view().position_changed
    }

    fn size_changed(&self) -> &Event {
        &self.base_view().size_changed
    }

    fn __after_setup_event(&self) -> &OnceEvent {
        &self.base_view().after_setup
    }
}

pub trait AfterSetup {
    fn after_setup(self: Own<Self>, action: impl FnOnce(Weak<Self>) + Send + 'static) -> Own<Self>;
}

impl<T: ?Sized + View + 'static> AfterSetup for T {
    fn after_setup(self: Own<Self>, action: impl FnOnce(Weak<Self>) + Send + 'static) -> Own<Self> {
        let weak = self.weak();
        self.base_view().after_setup.sub(move || {
            action(weak);
        });
        self
    }
}
