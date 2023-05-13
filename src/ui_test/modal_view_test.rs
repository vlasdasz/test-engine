use async_trait::async_trait;
use test_engine::{
    from_main,
    gm::{flat::Size, Color},
    on_main,
};
use tokio::sync::oneshot::{channel, Receiver, Sender};
use ui::{
    refs::{Own, Weak},
    view, ModalView, SubView, ViewData, ViewSetup, ViewTest,
};
use ui_views::{async_link_button, link_button, Button, Label, TextField};

#[view]
struct ModalTestView {
    button: SubView<Button>,
    text:   SubView<TextField>,
    sender: Option<Sender<u32>>,
}

impl ViewSetup for ModalTestView {
    fn setup(mut self: Weak<Self>) {
        self.set_color(Color::WHITE);
        self.button.set_text("Tap").place.size(100, 20).center();

        self.text.place.size(100, 20).l(0);

        link_button!(self, button, hide_modal);
    }
}

#[async_trait]
impl ModalView<u32> for ModalTestView {
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

    fn result(self: Weak<Self>) -> u32 {
        self.text.text().parse().unwrap()
    }
}

#[view]
struct ModalViewTestContainer {
    button: SubView<Button>,
    label:  SubView<Label>,
}

impl ModalViewTestContainer {
    async fn on_tap(mut self: Weak<Self>) {
        let result = from_main(|| {
            let modal = Own::<ModalTestView>::default();
            modal.show_modally()
        })
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
