use test_engine::gm::flat::Size;
use ui::{refs::Weak, view, Event, ModalView, SubView, ViewSetup};
use ui_views::{link_button, Button, Label, TextField};

#[view]
struct ModalTestView {
    button:      SubView<Button>,
    input_label: SubView<Label>,
    text_field:  SubView<TextField>,
    event:       Event<u32>,
}

impl ModalTestView {
    fn on_tap(self: Weak<Self>) {
        self.hide_modal(
            self.text_field.text().parse::<u32>().unwrap() + self.input_label.text().parse::<u32>().unwrap(),
        )
    }
}

impl ViewSetup for ModalTestView {
    fn setup(mut self: Weak<Self>) {
        self.button.set_text("Tap").place.size(100, 20).center();
        self.input_label.place.size(100, 20).tr(0);
        self.text_field.place.size(100, 20).l(0);

        link_button!(self, button, on_tap);
    }
}

impl ModalView<u32, u32> for ModalTestView {
    fn modal_event(&self) -> &Event<u32> {
        &self.event
    }

    fn modal_size() -> Size {
        (400, 400).into()
    }

    fn setup_input(mut self: Weak<Self>, input: u32) {
        self.input_label.set_text(input);
    }
}

#[view]
struct ModalViewTestContainer {
    button:     SubView<Button>,
    label:      SubView<Label>,
    text_field: SubView<TextField>,
}

impl ModalViewTestContainer {
    fn on_tap(mut self: Weak<Self>) {
        let input = self.text_field.text().parse().unwrap();
        ModalTestView::show_modally(input, move |result| {
            self.label.set_text(result);
        });
    }
}

impl ViewSetup for ModalViewTestContainer {
    fn setup(mut self: Weak<Self>) {
        self.button.set_text("Tap").place.size(100, 50);
        self.label.set_text("Nothing").place.size(100, 50).tr(0);
        self.text_field.place.size(100, 50).br(0);
        link_button!(self, button, on_tap);
    }
}

#[ignore]
#[test]
fn test() {
    test_engine::ViewApp::<ModalViewTestContainer>::start();
}
