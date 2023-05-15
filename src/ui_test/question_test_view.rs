use test_engine::{gm::flat::Size, on_main};
use ui::{refs::Weak, view, SubView, ViewSetup, ViewTest};
use ui_views::{async_link_button, link_button, Button, Label, Question};

#[view]
struct QuestionTestView {
    button:       SubView<Button>,
    async_button: SubView<Button>,
    label:        SubView<Label>,
}

impl QuestionTestView {
    fn on_button_tap(mut self: Weak<Self>) {
        Question::ask("Prokpudak prokpudok??", move |result| {
            self.label.set_text(result);
        });
    }

    async fn on_async_tap(mut self: Weak<Self>) {
        let answer = Question::ask_async("Asynk okokok?").await;
        on_main(move || {
            self.label.set_text(answer);
        });
    }
}

impl ViewSetup for QuestionTestView {
    fn setup(mut self: Weak<Self>) {
        self.label.place.size(200, 50).tr(0);

        self.async_button.set_text("Async ask").place.size(200, 50).br(0);
        async_link_button!(self, async_button, on_async_tap);

        self.button.set_text("Ask question").place.size(200, 50);
        link_button!(self, button, on_button_tap);
    }
}

impl ViewTest for QuestionTestView {
    fn test_size() -> Size
    where Self: Sized {
        (1000, 1000).into()
    }
}

#[ignore]
#[test]
fn test() {
    test_engine::ViewApp::<QuestionTestView>::start();
}
