use std::any::type_name;

use gm::{Color, ToF32};
use refs::{weak_from_ref, Weak};
use ui_proc::view;

use crate::{
    has_data::HasText, view::ViewData, Anchor, Container, HasTitle, InputView, Setup, TextAlignment,
    ViewSubviews,
};
mod test_engine {
    pub(crate) use educe;
    pub(crate) use refs;

    pub(crate) use crate as ui;
}

use crate::Label;

#[view]
pub struct Labeled<T: InputView + Default + 'static> {
    pub input: Weak<T>,

    #[educe(Default = 0.5)]
    label_input_ratio: f32,

    #[init]
    label: Label,
}

impl<T: InputView + Default> Labeled<T> {
    pub fn set_label_input_ratio(mut self: Weak<Self>, ratio: impl ToF32) {
        self.label_input_ratio = ratio.to_f32();
        self.input.remove_from_superview();
        self.layout_input();
    }

    fn layout_input(mut self: Weak<Self>) {
        let label_ratio = 1.0 - self.label_input_ratio;
        let input_ratio = self.label_input_ratio;

        self.label.place().clear().tlb(0).relative(Anchor::Width, self, label_ratio);

        if type_name::<T>() == "ui::views::basic::switch::Switch" {
            let mut container = self.add_view::<Container>();
            container.set_color(Color::WHITE);

            self.input = container.add_view::<T>();
            self.input.place().size(80, 46).center_y().r(10);

            container.place().trb(0).relative(Anchor::Width, self, input_ratio);
        } else if type_name::<T>().contains("number_view::NumberView<") {
            let mut container = self.add_view::<Container>();
            container.set_color(Color::WHITE);

            self.input = container.add_view::<T>();
            self.input.place().size(60, 100).center_y().r(10);

            container.place().trb(0).relative(Anchor::Width, self, input_ratio);
        } else {
            self.input = self.add_view::<T>();
            self.input.place().trb(0).relative(Anchor::Width, self, input_ratio);
        }
    }
}

impl<T: InputView + Default> Setup for Labeled<T> {
    fn setup(mut self: Weak<Self>) {
        self.__view_base.view_label += &format!(": {}", type_name::<T>());

        self.label.set_alignment(TextAlignment::Left);

        self.layout_input();
    }
}

impl<T: InputView + Default> HasTitle for Labeled<T> {
    fn title(&self) -> &str {
        self.label.text()
    }

    fn set_title(&mut self, title: &str) {
        self.label.set_text(title);
    }
}

impl<T: InputView + Default> InputView for Labeled<T> {
    fn set_text(&mut self, text: &str) {
        self.input.set_text(text);
    }

    fn text(&self) -> &str {
        self.input.text()
    }

    fn enable_editing(&mut self) {
        self.input.enable_editing();
    }

    fn disable_editing(&mut self) {
        self.input.disable_editing();
    }

    fn as_input_view(&self) -> Weak<dyn InputView> {
        weak_from_ref(self as &dyn InputView)
    }
}
