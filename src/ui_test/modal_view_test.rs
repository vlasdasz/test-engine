use async_trait::async_trait;
use test_engine::{
    from_main,
    gm::{flat::Size, Color},
    on_main,
};
use tokio::sync::oneshot::{channel, Receiver, Sender};
use ui::{refs::Weak, view, ModalView, SubView, ViewData, ViewSetup, ViewTest};
use ui_views::{async_link_button, link_button, Button, Label, TextField};

#[view]
struct ModalTestView {
    button:      SubView<Button>,
    input_label: SubView<Label>,
    text_field:  SubView<TextField>,
    sender:      Option<Sender<u32>>,
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
        self.set_color(Color::WHITE);
        self.button.set_text("Tap").place.size(100, 20).center();
        self.input_label.place.size(100, 20).tr(0);
        self.text_field.place.size(100, 20).l(0);

        link_button!(self, button, on_tap);
    }
}

#[async_trait]
impl ModalView<u32, u32> for ModalTestView {
    fn modal_size() -> Size {
        (400, 400).into()
    }

    fn send(&mut self) -> Sender<u32> {
        self.sender.take().unwrap()
    }

    fn recv(&mut self) -> Receiver<u32> {
        let (s, r) = channel();
        self.sender = s.into();
        r
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
    async fn on_tap(mut self: Weak<Self>) {
        let result = from_main(move || ModalTestView::show_modally(self.text_field.text().parse().unwrap()))
            .await
            .await
            .unwrap();

        on_main(move || {
            self.label.set_text(result);
        });
    }
}

impl ViewSetup for ModalViewTestContainer {
    fn setup(mut self: Weak<Self>) {
        self.button.set_text("Tap").place.size(100, 50);
        self.label.set_text("Nothing").place.size(100, 50).tr(0);
        self.text_field.place.size(100, 50).br(0);
        async_link_button!(self, button, on_tap);
    }
}

impl ViewTest for ModalViewTestContainer {
    fn test_size() -> Size
    where Self: Sized {
        (1000, 1000).into()
    }
}

#[ignore]
#[test]
fn test() {
    test_engine::ViewApp::<ModalViewTestContainer>::start();
}
