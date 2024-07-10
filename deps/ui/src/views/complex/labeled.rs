use std::any::type_name;

use gm::Color;
use refs::{weak_from_ref, Weak};
use ui_proc::view;

use crate::{
    has_data::HasText, view::ViewData, Anchor, Container, HasTitle, InputView, TextAlignment, ViewSetup,
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

    #[init]
    label: Label,
}

impl<T: InputView + Default> ViewSetup for Labeled<T> {
    fn setup(mut self: Weak<Self>) {
        self.view_label += &format!(": {}", type_name::<T>());

        self.label.place().tlb(0).relative(Anchor::Width, self, 0.6);
        self.label.set_alignment(TextAlignment::Left);

        if type_name::<T>() == "ui::views::basic::switch::Switch" {
            let mut container = self.add_view::<Container>();
            container.set_color(Color::WHITE);

            self.input = container.add_view::<T>();
            self.input.place().size(80, 46).center_y().r(10);

            container.place().trb(0).relative(Anchor::Width, self, 0.4);
        } else if type_name::<T>().contains("number_view::NumberView<") {
            let mut container = self.add_view::<Container>();
            container.set_color(Color::WHITE);

            self.input = container.add_view::<T>();
            self.input.place().size(60, 100).center_y().r(10);

            container.place().trb(0).relative(Anchor::Width, self, 0.4);
        } else {
            self.input = self.add_view::<T>();
            self.input.place().trb(0).relative(Anchor::Width, self, 0.4);
        }
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
